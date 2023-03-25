/// An item copied to the clipboard
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use chrono::Utc;
// use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io;
use thiserror::Error;

pub type Key = [u8; 32];

#[derive(Error, Debug)]
pub enum EntryError {
    #[error("error decoding the entry: {0}")]
    Decode(String),
    #[error("error encoding the entry: {0}")]
    Encode(String),
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
