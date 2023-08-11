use std::array::TryFromSliceError;

use crate::_rename::{secp256k1_schnorrsig_sign32, secp256k1_schnorrsig_verify};
use crate::context::Context;
use crate::errors::ConversionError;
use crate::keys::{Error as KeyError, KeyPair, XOnlyPublicKey};
use crate::scalar::Scalar;

#[derive(Debug, Clone)]
/// Errors in Schnorr signature operations
pub enum Error {
    /// Invalid message length
    InvalidMessageLength,
    /// Error with key operations
    Key(KeyError),
    /// Error occurred during a try from operation
    TryFrom(String),
    /// Error converting a scalar
    Conversion(ConversionError),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Self {
        Error::TryFrom(e.to_string())
    }
}

impl From<KeyError> for Error {
    fn from(e: KeyError) -> Self {
        Error::Key(e)
    }
}

/**
Signature is a wrapper around libsecp256k1's secp256k1_schnorr_signature struct.
*/
pub struct Signature {
    /// The wrapped libsecp256k1 signature
    pub data: [u8; 64],
}

impl Signature {
    /// Construct an Schnorr signature
    pub fn new(hash: &[u8], sec_key: &Scalar) -> Result<Self, Error> {
        if hash.len() != 32 {
            return Err(Error::InvalidMessageLength);
        }
        let mut sig = Self { data: [0; 64] };
        let keypair = KeyPair::new(sec_key)?;
        let context = Context::default();

        if unsafe {
            secp256k1_schnorrsig_sign32(
                context.context,
                sig.data.as_mut_ptr(),
                hash.as_ptr(),
                &keypair.key,
                std::ptr::null::<::std::os::raw::c_void>() as *const u8,
            )
        } == 0
        {
            return Err(Error::Key(KeyError::InvalidSecretKey));
        }
        Ok(sig)
    }

    /// Verify an Schnorr signature
    pub fn verify(&self, hash: &[u8], pub_key: &XOnlyPublicKey) -> bool {
        let context = Context::default();
        1 == unsafe {
            secp256k1_schnorrsig_verify(
                context.context,
                self.data.as_ptr(),
                hash.as_ptr(),
                hash.len(),
                &pub_key.key,
            )
        }
    }

    /// Returns the signature's deserialized underlying data
    pub fn to_bytes(&self) -> [u8; 64] {
        self.data
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = Error;
    /// Create an Schnorr signature given a slice of signed data.
    /// Note it also serializes the data in compact (64 byte) format
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let data: [u8; 64] = input[0..].try_into()?;
        Ok(Signature { data })
    }
}

impl From<[u8; 64]> for Signature {
    fn from(data: [u8; 64]) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;
    use sha2::{Digest, Sha256};

    #[test]
    fn signature() {
        // Generate a secret and public key
        let mut rnd = OsRng::default();
        let sec_key = Scalar::random(&mut rnd);
        let pub_key = XOnlyPublicKey::new(&sec_key).unwrap();

        // Instead of signing a message directly, must sign a 32-byte hash of it.
        let msg = b"Hello, world!";
        let mut hasher = Sha256::new();
        hasher.update(msg);
        let msg_hash = hasher.finalize();
        // Generate a Schnorr signature
        let sig = Signature::new(&msg_hash, &sec_key).unwrap();

        // Verify the generated signature is valid using the msg_hash and corresponding public key
        assert!(sig.verify(&msg_hash, &pub_key));

        let bytes = sig.to_bytes();

        let sig2 = Signature::try_from(&bytes[..]).unwrap();
        assert!(sig2.verify(&msg_hash, &pub_key));

        let sig3 = Signature::from(bytes);
        assert!(sig3.verify(&msg_hash, &pub_key));
    }
}
