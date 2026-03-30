//! A module for handling file hashes in Kitty Keep.

use std::{
    borrow::{Borrow, BorrowMut},
    fmt,
    ops::Deref,
    str::FromStr,
};

use hex::FromHex;

/// A wrapper around a 32-byte hash, typically used for hashes in Kitty Keep.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileHash([u8; 32]);

impl FileHash {
    /// Creates a new `FileHash` from a 32-byte array.
    pub const fn new(hash: [u8; 32]) -> Self {
        Self(hash)
    }

    /// Returns the hash as a byte array reference.
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Returns the hash as a byte slice.
    pub const fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// Returns the hexadecimal representation of the byte at the specified index.
    pub fn byte_hex(&self, index: usize) -> String {
        format!("{:02x}", self.0[index])
    }
}

impl From<[u8; 32]> for FileHash {
    fn from(bytes: [u8; 32]) -> Self {
        Self::new(bytes)
    }
}

impl From<FileHash> for [u8; 32] {
    fn from(hash: FileHash) -> Self {
        hash.0
    }
}

impl fmt::Display for FileHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl FromHex for FileHash {
    type Error = hex::FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        Ok(Self::from(<[u8; 32]>::from_hex(hex)?))
    }
}

impl FromStr for FileHash {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_hex(s)
    }
}

impl AsRef<[u8; 32]> for FileHash {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl AsRef<[u8]> for FileHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for FileHash {
    type Target = [u8; 32];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<[u8; 32]> for FileHash {
    fn borrow(&self) -> &[u8; 32] {
        &self.0
    }
}

impl BorrowMut<[u8; 32]> for FileHash {
    fn borrow_mut(&mut self) -> &mut [u8; 32] {
        &mut self.0
    }
}
