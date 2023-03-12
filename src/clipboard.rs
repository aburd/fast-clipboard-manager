use crate::config::Config;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use chrono::Utc;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use thiserror::Error;

const DEFAULT_MAX_ENTRIES: usize = 5;

pub type Key = [u8; 32];

#[derive(Error, Debug)]
pub enum EntryError {
    #[error("error decoding the entry: {0}")]
    Decode(String),
    #[error("error encoding the entry: {0}")]
    Encode(String),
    #[error("Could not serialize entry: {0}")]
    CantSerialize(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("unknown data store error: {0}")]
    Unknown(String),
}

impl From<io::Error> for EntryError {
    fn from(e: io::Error) -> EntryError {
        EntryError::Unknown(e.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub enum EntryKind {
    Text,
    Image,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Entry {
    bytes: Vec<u8>,
    kind: EntryKind,
    pub datetime: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct EncryptedEntry {
    ciphertext: Vec<u8>,
    nonce: Vec<u8>,
    kind: EntryKind,
}

impl EncryptedEntry {
    pub fn try_into_entry(self, key: &Key) -> Result<Entry, EntryError> {
        let cipher = ChaCha20Poly1305::new(key.into());
        let nonce = Nonce::clone_from_slice(&self.nonce[0..12]);
        let plaintext = cipher
            .decrypt(&nonce, self.ciphertext.as_ref())
            .map_err(|e| EntryError::Decode(e.to_string()))?;
        Ok(Entry::new(&plaintext, self.kind))
    }
}

impl Entry {
    pub fn new(bytes: &Vec<u8>, kind: EntryKind) -> Self {
        let dt = Utc::now();
        Entry {
            bytes: bytes.to_owned(),
            kind,
            datetime: dt.to_rfc3339(),
        }
    }

    pub fn encode(&self, key: &Key) -> Result<EncryptedEntry, EntryError> {
        let cipher = ChaCha20Poly1305::new(key.into());
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher
            .encrypt(&nonce, self.bytes.as_ref())
            .map_err(|e| EntryError::Encode(e.to_string()))?;
        Ok(EncryptedEntry {
            ciphertext,
            nonce: nonce.as_slice().into(),
            kind: self.kind,
        })
    }

    pub fn content(&self) -> &[u8] {
        &self.bytes
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            EntryKind::Text => {
                write!(f, "{}", String::from_utf8(self.bytes.clone()).unwrap())
            }
            EntryKind::Image => {
                write!(f, "[{}] ENTRY[IMAGE]: {:?}", self.datetime, self.bytes)
            }
        }
    }
}

#[derive(Debug)]
pub struct Clipboard {
    storage: File,
    /// Clipboard entries. Stored as a vector because I am uncreative
    entries: Vec<Entry>,
    /// How many entries are allowed in the Clipboard
    /// A new copy will always force the oldest from the clipboard
    max_entries: usize,
    key: Key,
}

pub fn generate_encryption_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng).into()
}

impl Clipboard {
    pub fn new(storage: File, key: Key) -> Self {
        Clipboard {
            storage,
            entries: vec![],
            max_entries: DEFAULT_MAX_ENTRIES,
            key,
        }
    }

    /// Persists current Clipboard to the Writer
    pub fn save(&mut self) -> Result<(), EntryError> {
        let encoded: Result<Vec<EncryptedEntry>, EntryError> = self
            .entries
            .clone()
            .into_iter()
            .map(|entry| entry.encode(&self.key))
            .collect();
        let serialized = serde_json::to_string(&encoded.unwrap())
            .map_err(|e| EntryError::CantSerialize(e.to_string()))?;

        self.storage.set_len(0)?;
        self.storage.seek(SeekFrom::Start(0))?;
        self.storage.write_all(&serialized.as_bytes())?;
        self.storage.flush()?;
        Ok(())
    }

    /// Loads all from Reader into current Clipboard
    pub fn load(&mut self) -> Result<(), EntryError> {
        let mut buf = String::new();
        self.storage.read_to_string(&mut buf)?;
        debug!("load buf: [{}]", buf);
        if buf.is_empty() {
            info!("initializing new empty clipboard");
            buf = serde_json::to_string::<Vec<Entry>>(&self.entries)
                .map_err(|e| EntryError::Decode(e.to_string()))?;
            self.save()?;
        }
        let decoded = serde_json::from_str::<Vec<EncryptedEntry>>(&buf)
            .map_err(|e| EntryError::Decode(format!("serde: {}", e.to_string())))?;
        self.entries = decoded
            .into_iter()
            .map(|encrypted| encrypted.try_into_entry(&self.key))
            .collect::<Result<Vec<Entry>, EntryError>>()?;

        debug!("loaded {} clipboard entries", self.entries.len());
        Ok(())
    }

    /// idx will wrap to length of entries in Clipboard
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
    pub fn add_entry(&mut self, entry: Entry) -> Result<(), EntryError> {
        if let Some(idx) = self.entries.iter().position(|e| e.bytes == entry.bytes) {
            self.entries.swap(0, idx);
        } else {
            self.entries.insert(0, entry);
            if self.entries.len() > self.max_entries {
                self.clip_entries_to_max_size();
            }
        }
        Ok(())
    }

    pub fn remove_entry(&mut self, idx: usize) -> Result<(), EntryError> {
        if idx >= self.entries.len() {
            return Err(EntryError::InvalidOperation(format!(
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

pub fn get_clipboard(config: &Config) -> Result<Clipboard, Box<dyn Error>> {
    let storage = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&config.config_dir.join("entries.json"))
        .unwrap();
    // TODO: setup key gracefully
    let key: &[u8; 32] = b"Thisisakeyof32bytesThisisakeyof3";
    let mut clipboard = Clipboard::new(storage, key.to_owned());
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
    fn can_encode_and_decode_entry() {
        let bytes = vec![1, 2, 3, 4];
        let entry = Entry::new(&bytes, EntryKind::Text);
        let encrypted = entry.encode(KEY).unwrap();
        let decoded = encrypted.try_into_entry(KEY).unwrap();
        assert_eq!(entry.bytes, decoded.bytes);
        assert_eq!(entry.kind, decoded.kind);
    }

    #[test]
    fn can_add_entry() {
        let f = new_file("");
        let mut clipboard = Clipboard::new(f, KEY.to_owned());
        assert_eq!(clipboard.entries.len(), 0);
        clipboard
            .add_entry(Entry::new(&vec![], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
    }

    #[test]
    fn can_remove_entry_from_clipboard() {
        let f = new_file("");
        let mut clipboard = Clipboard::new(f, KEY.to_owned());
        clipboard
            .add_entry(Entry::new(&vec![], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
        clipboard.remove_entry(0).unwrap();
        assert_eq!(clipboard.entries.len(), 0);
    }

    #[test]
    fn doesnt_change_length_over_max() {
        let f = new_file("");
        let mut clipboard = Clipboard::new(f, KEY.to_owned());
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
    fn replaces_oldest_entry() {
        let f = new_file("");
        let mut clipboard = Clipboard::new(f, KEY.to_owned());
        clipboard.max_entries = 1;
        clipboard
            .add_entry(Entry::new(&vec![1], EntryKind::Text))
            .unwrap();
        clipboard
            .add_entry(Entry::new(&vec![2], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries[0].bytes, vec![2]);
    }

    #[test]
    fn lastest_entry_is_first() {
        let f = new_file("");
        let mut clipboard = Clipboard::new(f, KEY.to_owned());
        clipboard.max_entries = 2;
        clipboard
            .add_entry(Entry::new(&vec![1], EntryKind::Text))
            .unwrap();
        clipboard
            .add_entry(Entry::new(&vec![2], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries[0].bytes, vec![2]);
        clipboard
            .add_entry(Entry::new(&vec![3], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries[0].bytes, vec![3]);
    }

    #[test]
    fn load_works() {
        let bytes = vec![1, 2, 3, 4];
        let entry = Entry::new(&bytes, EntryKind::Text);
        let encoded = entry.encode(KEY).unwrap();
        let json_s = serde_json::to_string(&vec![encoded]).unwrap();
        let f = new_file(&json_s);
        let mut clipboard = Clipboard::new(f, KEY.to_owned());
        clipboard.load().unwrap();
        assert_eq!(clipboard.entries.len(), 1);
    }
}
