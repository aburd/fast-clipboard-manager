use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::io::{self, BufReader, BufWriter, Read, Write};
use thiserror::Error;

const DEFAULT_MAX_ENTRIES: usize = 100;
pub type Key = [u8; 32];

#[derive(Error, Debug)]
pub enum EntryError {
    #[error("error decoding the entry: {}", ".0")]
    Decode(String),
    #[error("error encoding the entry: {}", ".0")]
    Encode(String),
    #[error("Could not serialize entry: {}", ".0")]
    CantSerialize(String),
    #[error("Invalid operation: {}", ".0")]
    InvalidOperation(String),
    #[error("unknown data store error: {}", ".0")]
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
    datetime: String,
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
}

pub struct Clipboard<'a, R: Read, W: Write> {
    /// Used to load the entries into memory. I.E. a file
    reader: BufReader<R>,
    /// Used to persist the entries. I.E. a file
    writer: BufWriter<W>,
    /// Clipboard entries. Stored as a vector because I am uncreative
    entries: Vec<Entry>,
    /// How many entries are allowed in the Clipboard
    /// A new copy will always force the oldest from the clipboard
    max_entries: usize,
    key: &'a Key,
}

pub fn generate_encryption_key() -> Key {
    ChaCha20Poly1305::generate_key(&mut OsRng).into()
}

impl<'a, R: Read, W: Write> Clipboard<'a, R, W> {
    pub fn new(reader: BufReader<R>, writer: BufWriter<W>, key: &'a Key) -> Self {
        Clipboard {
            reader,
            writer,
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
            .map(|entry| entry.encode(self.key))
            .collect();
        let serialized = serde_json::to_vec(&encoded.unwrap())
            .map_err(|e| EntryError::CantSerialize(e.to_string()))?;
        self.writer.write_all(&serialized)?;
        Ok(())
    }

    /// Loads all from Reader into current Clipboard
    pub fn load(&mut self) -> Result<(), EntryError> {
        let mut buf = String::new();
        self.reader.read_to_string(&mut buf)?;
        let decoded = serde_json::from_str::<Vec<EncryptedEntry>>(&buf)
            .map_err(|e| EntryError::Decode(e.to_string()))?;
        self.entries = decoded
            .into_iter()
            .map(|encrypted| encrypted.try_into_entry(self.key))
            .collect::<Result<Vec<Entry>, EntryError>>()?;

        Ok(())
    }

    /// idx will wrap to length of entries in Clipboard
    pub fn get_entry(&mut self, idx: usize) -> &Entry {
        &self.entries[idx % self.entries.len()]
    }

    /// Clips off any entries at beginning
    pub fn add_entry(&mut self, entry: Entry) -> Result<(), EntryError> {
        self.entries.push(entry);
        if self.entries.len() > self.max_entries {
            self.clip_entries_to_max_size();
        }
        self.save()?;
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
        self.entries = self
            .entries
            .drain((self.entries.len() - self.max_entries)..)
            .collect();
    }
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
        let encrypted = entry.encode(&KEY).unwrap();
        let decoded = encrypted.try_into_entry(&KEY).unwrap();
        assert_eq!(entry.bytes, decoded.bytes);
        assert_eq!(entry.kind, decoded.kind);
    }

    #[test]
    fn can_add_entry() {
        let f = new_file("");
        let mut clipboard = Clipboard::new(BufReader::new(&f), BufWriter::new(&f), &KEY);
        assert_eq!(clipboard.entries.len(), 0);
        clipboard
            .add_entry(Entry::new(&vec![], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
    }

    #[test]
    fn can_remove_entry_from_clipboard() {
        let f = new_file("");
        let mut clipboard = Clipboard::new(BufReader::new(&f), BufWriter::new(&f), &KEY);
        clipboard
            .add_entry(Entry::new(&vec![], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
        clipboard.remove_entry(0).unwrap();
        assert_eq!(clipboard.entries.len(), 0);
    }

    #[test]
    fn removes_entries_over_max() {
        let f = new_file("");
        let mut clipboard = Clipboard::new(BufReader::new(&f), BufWriter::new(&f), &KEY);
        clipboard.max_entries = 1;
        clipboard
            .add_entry(Entry::new(&vec![1], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
        clipboard
            .add_entry(Entry::new(&vec![2], EntryKind::Text))
            .unwrap();
        assert_eq!(clipboard.entries.len(), 1);
        assert_eq!(clipboard.entries[0].bytes, vec![2]);
    }

    #[test]
    fn load_works() {
        let bytes = vec![1, 2, 3, 4];
        let entry = Entry::new(&bytes, EntryKind::Text);
        let encoded = entry.encode(&KEY).unwrap();
        let json_s = serde_json::to_string(&vec![encoded]).unwrap();
        let f = new_file(&json_s);
        let mut clipboard = Clipboard::new(BufReader::new(&f), BufWriter::new(&f), &KEY);
        clipboard.load().unwrap();
        assert_eq!(clipboard.entries.len(), 1);
    }
}
