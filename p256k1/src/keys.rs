use bs58;
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::array::TryFromSliceError;

use crate::_rename::{
    secp256k1_ec_pubkey_create, secp256k1_ec_pubkey_parse, secp256k1_ec_pubkey_serialize,
};
use crate::bindings::{secp256k1_pubkey, SECP256K1_EC_COMPRESSED};
use crate::context::Context;
use crate::errors::{Base58Error, ConversionError};
use crate::scalar::Scalar;

#[derive(Debug, Clone)]
/// Errors in ECDSA signature operations
pub enum Error {
    /// Error occurred due to invalid secret key
    InvalidSecretKey,
    /// Error occurred due to invalid public key
    InvalidPublicKey,
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

/**
PublicKey is a wrapper around libsecp256k1's secp256k1_pubkey struct.
*/
#[derive(Clone, Copy)]
pub struct PublicKey {
    /// The wrapped secp256k1_pubkey public key
    pub key: secp256k1_pubkey,
}

impl PublicKey {
    /// Construct a public key from a given secret key
    pub fn new(sec_key: &Scalar) -> Result<Self, Error> {
        let mut pub_key = Self {
            key: secp256k1_pubkey { data: [0; 64] },
        };
        let ctx = Context::default();
        if unsafe {
            secp256k1_ec_pubkey_create(ctx.context, &mut pub_key.key, sec_key.to_bytes().as_ptr())
        } == 0
        {
            return Err(Error::InvalidSecretKey);
        }
        Ok(pub_key)
    }

    /// Serialize the key to a compressed byte array
    pub fn to_bytes(&self) -> [u8; 33] {
        let ctx = Context::default();
        let mut bytes = [0u8; 33];
        let mut len = bytes.len();

        unsafe {
            secp256k1_ec_pubkey_serialize(
                ctx.context,
                bytes.as_mut_ptr(),
                &mut len,
                &self.key,
                SECP256K1_EC_COMPRESSED,
            );
        }

        bytes
    }
}

impl Debug for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("PublicKey")
            .field("data", &bs58::encode(self.key.data).into_string())
            .finish()
    }
}

impl Display for PublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", bs58::encode(self.to_bytes()).into_string())
    }
}

impl TryFrom<&str> for PublicKey {
    type Error = Error;
    /// Create a pubkey from the passed byte slice
    fn try_from(s: &str) -> Result<Self, self::Error> {
        match bs58::decode(s).into_vec() {
            Ok(bytes) => PublicKey::try_from(&bytes[..]),
            Err(e) => Err(Error::Conversion(ConversionError::Base58(
                Base58Error::Decode(e),
            ))),
        }
    }
}

impl TryFrom<&[u8]> for PublicKey {
    type Error = Error;
    /// Create a pubkey from the passed byte slice
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let mut pubkey = Self {
            key: secp256k1_pubkey { data: [0; 64] },
        };
        let ctx = Context::default();
        unsafe {
            match secp256k1_ec_pubkey_parse(
                ctx.context,
                &mut pubkey.key,
                input.as_ptr(),
                input.len(),
            ) {
                1 => Ok(pubkey),
                _ => Err(Error::InvalidPublicKey),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::{OsRng, RngCore};
    use sha2::{Digest, Sha256};
    use std::thread;

    #[test]
    fn pubkey_serde() {
        // Generate a secret and public key
        let mut rnd = OsRng::default();
        let sec_key = Scalar::random(&mut rnd);
        let pub_key = PublicKey::new(&sec_key).unwrap();

        //Serialize with try_from and deseriailze with to_bytes
        let pub_key_2 = PublicKey::try_from(pub_key.to_bytes().as_slice()).unwrap();
        assert_eq!(pub_key_2.to_bytes(), pub_key.to_bytes());
        assert_eq!(pub_key_2.key.data, pub_key.key.data);
    }
}
