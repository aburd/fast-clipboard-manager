use crate::entry::{EncryptedEntry, Entry, EntryError, EntryKind};

use chacha20poly1305::{
    aead::{KeyInit, OsRng},
    ChaCha20Poly1305,
};

use log::{debug, info};
use thiserror::Error;

/// Deals with reading/writing clipboard entries to storage (e.g. a File)
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

const DEFAULT_MAX_ENTRIES: usize = 5;

pub type Key = [u8; 32];

#[derive(Debug)]
pub struct ClipboardStorage {
    storage: File,
    /// ClipboardStorage entries. Stored as a vector because I am uncreative
    entries: Vec<Entry>,
    /// How many entries are allowed in the ClipboardStorage
    /// A new copy will always force the oldest from the clipboard
    max_entries: usize,
    key: Key,
}

pub fn generate_encryption_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng).into()
}

#[derive(Error, Debug)]
pub enum ClipboardStorageError {
    #[error("invalid operation on clipboard storage: {0}")]
    InvalidOperation(String),
    #[error("error with serialization: {0}")]
    Serialization(String),
    #[error("unknown data store error: {0}")]
    Unknown(String),
}

impl From<EntryError> for ClipboardStorageError {
    fn from(value: EntryError) -> Self {
        match value {
            EntryError::Decode(s) => {
                ClipboardStorageError::Serialization(format!("EntryError: {}", s))
            }
            EntryError::Encode(s) => {
                ClipboardStorageError::Serialization(format!("EntryError: {}", s))
            }
            EntryError::Unknown(s) => ClipboardStorageError::Unknown(format!("EntryError: {}", s)),
        }
    }
}

impl From<io::Error> for ClipboardStorageError {
    fn from(e: io::Error) -> Self {
        ClipboardStorageError::Unknown(e.to_string())
    }
}

impl ClipboardStorage {
    pub fn new(storage: File, key: Key) -> Self {
        ClipboardStorage {
            storage,
            entries: vec![],
            max_entries: DEFAULT_MAX_ENTRIES,
            key,
        }
    }

    /// Persists current ClipboardStorage to the Writer
    pub fn save(&mut self) -> Result<(), ClipboardStorageError> {
        let encoded: Result<Vec<EncryptedEntry>, EntryError> = self
            .entries
            .clone()
            .into_iter()
            .map(|entry| entry.encode(&self.key))
            .collect();
        let serialized = serde_json::to_string(&encoded.unwrap())
            .map_err(|e| ClipboardStorageError::Serialization(e.to_string()))?;

        self.storage.set_len(0)?;
        self.storage.seek(SeekFrom::Start(0))?;
        self.storage.write_all(&serialized.as_bytes())?;
        self.storage.flush()?;
        Ok(())
    }

    /// Loads all from Reader into current ClipboardStorage
    pub fn load(&mut self) -> Result<(), ClipboardStorageError> {
        let mut buf = String::new();
        self.storage.read_to_string(&mut buf)?;
        debug!("load buf: [{}]", buf);
        if buf.is_empty() {
            info!("initializing new empty clipboard");
            buf = serde_json::to_string::<Vec<Entry>>(&self.entries)
                .map_err(|e| ClipboardStorageError::Serialization(e.to_string()))?;
            self.save()?;
        }
        let decoded = serde_json::from_str::<Vec<EncryptedEntry>>(&buf)
            .map_err(|e| ClipboardStorageError::Serialization(e.to_string()))?;
        self.entries = decoded
            .into_iter()
            .map(|encrypted| encrypted.try_into_entry(&self.key))
            .collect::<Result<Vec<Entry>, EntryError>>()?;

        debug!("loaded {} clipboard entries", self.entries.len());
        Ok(())
    }

    /// idx will wrap to length of entries in ClipboardStorage
    pub fn get_entry(&self, idx: usize) -> &Entry {
        &self.entries[idx % self.entries.len()]
    }

    pub fn list_entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Clips off any entries at beginning
    pub fn add_entry(&mut self, entry: Entry) -> Result<(), ClipboardStorageError> {
        if let Some(idx) = self
            .entries
            .iter()
            .position(|e| entry.content() == e.content())
        {
            self.entries.swap(0, idx);
        } else {
            self.entries.insert(0, entry);
            if self.entries.len() > self.max_entries {
                self.clip_entries_to_max_size();
            }
        }
        Ok(())
    }

    pub fn remove_entry(&mut self, idx: usize) -> Result<(), ClipboardStorageError> {
        if idx >= self.entries.len() {
            return Err(ClipboardStorageError::InvalidOperation(format!(
                "Cannot remove entry at index: {}",
                idx
            )));
        }
        self.entries.remove(idx);
        self.save()?;
        Ok(())
    }

    fn clip_entries_to_max_size(&mut self) {
        self.entries = self.entries.drain(0..(self.entries.len() - 1)).collect();
    }
}

pub fn get_clipboard(dir: &PathBuf) -> Result<ClipboardStorage, Box<dyn Error>> {
    let storage = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(dir.join("entries.json"))
        .unwrap();
    // TODO: setup key gracefully
    let key: &[u8; 32] = b"Thisisakeyof32bytesThisisakeyof3";
    let mut clipboard = ClipboardStorage::new(storage, key.to_owned());
    clipboard.load()?;
    Ok(clipboard)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{File, OpenOptions};

    const KEY: &[u8; 32] = b"Thisisakeyof32bytesThisisakeyof3";

    fn new_file(content: &str) -> File {
        let tmp_file = temp_file::with_contents(content.as_bytes());
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(tmp_file.path())
            .unwrap()
    }

    #[test]
    fn test_store_can_encode_and_decode_entry() {
        let bytes = vec![1, 2, 3, 4];
        let entry = Entry::new(&bytes, EntryKind::Text);
        let encrypted = entry.encode(KEY).unwrap();
        let decoded = encrypted.try_into_entry(KEY).unwrap();
        assert_eq!(entry.content(), decoded.content());
    }

    #[test]
    fn test_store_can_add_entry() {
        let f = new_file("");
        let mut clipboard = ClipboardStorage::new(f, KEY.to_owned());
        assert_eq!(clipboard.entries.len(), 0);
        clipboard
            .add_entry(Entry::new(&vec![], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
    }

    #[test]
    fn test_store_can_remove_entry_from_clipboard() {
        let f = new_file("");
        let mut clipboard = ClipboardStorage::new(f, KEY.to_owned());
        clipboard
            .add_entry(Entry::new(&vec![], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
        clipboard.remove_entry(0).unwrap();
        assert_eq!(clipboard.entries.len(), 0);
    }

    #[test]
    fn test_store_doesnt_change_length_over_max() {
        let f = new_file("");
        let mut clipboard = ClipboardStorage::new(f, KEY.to_owned());
        clipboard.max_entries = 1;
        clipboard
            .add_entry(Entry::new(&vec![1], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
        clipboard
            .add_entry(Entry::new(&vec![2], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
    }

    #[test]
    fn test_store_replaces_oldest_entry() {
        let f = new_file("");
        let mut clipboard = ClipboardStorage::new(f, KEY.to_owned());
        clipboard.max_entries = 1;
        clipboard
            .add_entry(Entry::new(&vec![1], EntryKind::Text))
            .unwrap();
        clipboard
            .add_entry(Entry::new(&vec![2], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries[0].content(), vec![2]);
    }

    #[test]
    fn test_store_lastest_entry_is_first() {
        let f = new_file("");
        let mut clipboard = ClipboardStorage::new(f, KEY.to_owned());
        clipboard.max_entries = 2;
        clipboard
            .add_entry(Entry::new(&vec![1], EntryKind::Text))
            .unwrap();
        clipboard
            .add_entry(Entry::new(&vec![2], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries[0].content(), vec![2]);
        clipboard
            .add_entry(Entry::new(&vec![3], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries[0].content(), vec![3]);
    }

    #[test]
    fn test_store_load_works() {
        let bytes = vec![1, 2, 3, 4];
        let entry = Entry::new(&bytes, EntryKind::Text);
        let encoded = entry.encode(KEY).unwrap();
        let json_s = serde_json::to_string(&vec![encoded]).unwrap();
        let f = new_file(&json_s);
        let mut clipboard = ClipboardStorage::new(f, KEY.to_owned());
        clipboard.load().unwrap();
        assert_eq!(clipboard.entries.len(), 1);
    }
}
