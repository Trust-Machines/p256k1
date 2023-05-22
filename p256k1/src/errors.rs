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
/// Errors when converting points
pub enum ConversionError {
    /// Error decompressing a point into a field element
    BadFieldElement,
    /// Error decompressing a point into a group element
    BadGroupElement,
    /// Error with data size
    WrongNumberOfBytes(usize),
    /// Error converting a base58 string to bytes
    Base58(Base58Error),
}
