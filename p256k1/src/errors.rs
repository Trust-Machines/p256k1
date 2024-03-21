use core::{
    cmp::PartialEq,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use serde::{Deserialize, Serialize};

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

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Base58Error {}
impl std::error::Error for ConversionError {}
