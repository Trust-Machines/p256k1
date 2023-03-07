use crate::bindings::{
    secp256k1_ec_pubkey_create, secp256k1_ecdsa_sign, secp256k1_ecdsa_signature,
    secp256k1_ecdsa_verify, secp256k1_pubkey,
};
use crate::context::Context;
use crate::scalar::Scalar;
use std::array::TryFromSliceError;

#[derive(Debug)]
/// Errors in ECDSA signature operations
pub enum Error {
    /// Error occurred due to invalid secret key
    InvalidSecretKey,
    /// Error occurred during a try from operation
    TryFrom(String),
}

/**
PubKey is a wrapper around libsecp256k1's secp256k1_pubkey struct.
*/
pub struct PubKey {
    key: secp256k1_pubkey,
}

impl PubKey {
    /// Construct a public key from a given secret key
    pub fn new(ctx: &Context, sec_key: &Scalar) -> Result<Self, Error> {
        let mut pub_key = Self {
            key: secp256k1_pubkey { data: [0; 64] },
        };
        if unsafe {
            secp256k1_ec_pubkey_create(ctx.context, &mut pub_key.key, sec_key.to_bytes().as_ptr())
        } == 0
        {
            return Err(Error::InvalidSecretKey);
        }
        Ok(pub_key)
    }
}

/**
Signature is a wrapper around libsecp256k1's secp256k1_ecdsa_signature struct.
*/
pub struct Signature {
    /// The wrapped libsecp256k1 signature
    pub signature: secp256k1_ecdsa_signature,
}

impl Signature {
    /// Construct an ECDSA signature
    pub fn new(ctx: &Context, hash: &[u8], sec_key: &Scalar) -> Result<Self, Error> {
        let mut sig = Self {
            signature: secp256k1_ecdsa_signature { data: [0; 64] },
        };
        if unsafe {
            secp256k1_ecdsa_sign(
                ctx.context,
                &mut sig.signature,
                hash.as_ptr(),
                sec_key.to_bytes().as_ptr(),
                None,
                std::ptr::null_mut::<::std::os::raw::c_void>(),
            )
        } == 0
        {
            return Err(Error::InvalidSecretKey);
        }
        Ok(sig)
    }

    /// Verify an ECDSA signature
    pub fn verify(&self, ctx: &Context, hash: &[u8], pub_key: &PubKey) -> bool {
        1 == unsafe {
            secp256k1_ecdsa_verify(ctx.context, &self.signature, hash.as_ptr(), &pub_key.key)
        }
    }

    /// Return the underlying bytes within a signature
    pub fn to_bytes(&self) -> [u8; 64] {
        self.signature.data
    }
}

impl From<[u8; 64]> for Signature {
    fn from(data: [u8; 64]) -> Self {
        Self {
            signature: secp256k1_ecdsa_signature { data },
        }
    }
}

impl TryFrom<&[u8]> for Signature {
    type Error = Error;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let data: [u8; 64] = data[0..64]
            .try_into()
            .map_err(|e: TryFromSliceError| Error::TryFrom(e.to_string()))?;
        Ok(Signature::from(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::{OsRng, RngCore};
    use sha3::{Digest, Sha3_256};

    #[test]
    fn signature_generation() {
        let ctx = Context::default();

        // Generate a secret and public key
        let mut rnd = OsRng::default();
        let sec_key = Scalar::random(&mut rnd);
        let pub_key = PubKey::new(&ctx, &sec_key).unwrap();

        // Instead of signing a message directly, must sign a 32-byte hash of it.
        let msg = b"Hello, world!";
        let mut hasher = Sha3_256::new();
        hasher.update(msg);
        let msg_hash = hasher.finalize();
        // Generate a ECDSA signature
        let sig = Signature::new(&ctx, &msg_hash, &sec_key).unwrap();

        // Verify the generated signature is valid using the msg_hash and corresponding public key
        assert!(sig.verify(&ctx, &msg_hash, &pub_key));
    }

    #[test]
    fn from() {
        let mut rng = OsRng::default();
        let mut bytes = [0u8; 64];

        rng.fill_bytes(&mut bytes);

        let sig_from_struct = Signature {
            signature: secp256k1_ecdsa_signature { data: bytes },
        };

        let sig_from_slice = Signature::try_from(bytes.as_slice()).unwrap();
        let sig_from_array = Signature::from(bytes);

        assert_eq!(sig_from_struct.to_bytes(), sig_from_slice.to_bytes());
        assert_eq!(sig_from_struct.to_bytes(), sig_from_array.to_bytes());
    }

    #[test]
    fn to_bytes() {
        let mut rng = OsRng::default();
        let mut bytes = [0u8; 64];
        rng.fill_bytes(&mut bytes);

        let sig = Signature::from(bytes);
        assert_eq!(sig.to_bytes(), sig.signature.data);
    }
}
