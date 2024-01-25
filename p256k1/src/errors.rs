use bs58::{decode::Error as DecodeError, encode::Error as EncodeError};
use core::{
    cmp::PartialEq,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use serde::{Deserialize, Serialize};

/// Re-export of crate `bs58`'s decode error
pub type Base58DecodeError = DecodeError;
/// Re-export of crate `bs58`'s encode error
pub type Base58EncodeError = EncodeError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Base58-related errors
pub enum Base58Error {
    /// Error decoding
    Decode,
    /// Error encoding
    Encode,
}

impl Display for Base58Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Errors when performing conversion operations
pub enum ConversionError {
    /// Error decompressing a point into a field element
    BadFieldElement,
    /// Error decompressing a point into a group element
    BadGroupElement,
    /// Error with data size
    WrongNumberOfBytes(usize),
    /// Error converting a base58-related value
    Base58(Base58Error),
}
