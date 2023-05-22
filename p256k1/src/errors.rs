use bs58::{decode::Error as DecodeError, encode::Error as EncodeError};

/// Re-export of crate `bs58`'s decode error
pub type Base58DecodeError = DecodeError;
/// Re-export of crate `bs58`'s encode error
pub type Base58EncodeError = EncodeError;

#[derive(Debug, Clone)]
/// Base58-related errors
pub enum Base58Error {
    /// Error decoding
    Decode(Base58DecodeError),
    /// Error encoding
    Encode(Base58EncodeError),
}

#[derive(Debug, Clone)]
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
