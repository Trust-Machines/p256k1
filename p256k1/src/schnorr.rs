use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::array::TryFromSliceError;

use crate::_rename::{secp256k1_schnorrsig_sign32, secp256k1_schnorrsig_verify};
use crate::{
    context::Context,
    errors::ConversionError,
    keys::{Error as KeyError, KeyPair, XOnlyPublicKey},
    scalar::Scalar,
};

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
impl Display for Error {
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
#[derive(Debug, Clone)]
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

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.to_bytes())
    }
}

struct SignatureVisitor;

impl<'de> Visitor<'de> for SignatureVisitor {
    type Value = Signature;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("an array of bytes which represents two scalars on the secp256k1 curve")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Signature::try_from(value) {
            Ok(s) => Ok(s),
            Err(e) => Err(E::custom(format!("{:?}", e))),
        }
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut v = Vec::new();

        while let Ok(Some(x)) = seq.next_element() {
            v.push(x);
        }

        self.visit_bytes(&v)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Signature, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(SignatureVisitor)
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
    use rand_core::{OsRng, RngCore};
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
        let msg_hash = hasher.clone().finalize();
        // Generate a Schnorr signature
        let sig = Signature::new(&msg_hash, &sec_key).unwrap();

        // Verify the generated signature is valid using the msg_hash and corresponding public key
        assert!(sig.verify(&msg_hash, &pub_key));

        let bytes = sig.to_bytes();

        let sig2 = Signature::try_from(&bytes[..]).unwrap();
        assert!(sig2.verify(&msg_hash, &pub_key));

        let sig3 = Signature::from(bytes);
        assert!(sig3.verify(&msg_hash, &pub_key));

        // make sure that signing with a bad secret key fails
        let bad_sec_key = sec_key + Scalar::from(1);
        let bad_sig = Signature::new(&msg_hash, &bad_sec_key).unwrap();
        assert!(!bad_sig.verify(&msg_hash, &pub_key));

        // make sure that verifying with a bad public key fails
        let bad_pub_key = XOnlyPublicKey::new(&bad_sec_key).unwrap();
        assert!(!sig.verify(&msg_hash, &bad_pub_key));

        // make sure that verifying with a bad msg hash fails
        hasher.update("foo");
        let bad_msg_hash = hasher.finalize();
        assert!(!sig.verify(&bad_msg_hash, &pub_key));
    }

    #[test]
    fn custom_serde() {
        let mut rng = OsRng::default();
        let mut hash = [0u8; 32];
        rng.fill_bytes(&mut hash);
        let private_key = Scalar::random(&mut rng);
        let public_key = XOnlyPublicKey::new(&private_key).expect("failed to create public key");
        let sig = Signature::new(&hash, &private_key).expect("failed to create sig");

        assert!(sig.verify(&hash, &public_key));

        let ssig = serde_json::to_string(&sig).expect("failed to serialize");
        let dsig: Signature = serde_json::from_str(&ssig).expect("failed to deserialize");

        assert!(dsig.verify(&hash, &public_key));
    }
}
