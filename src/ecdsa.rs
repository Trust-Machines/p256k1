use crate::bindings::{
    secp256k1_ec_pubkey_create, secp256k1_ecdsa_sign, secp256k1_ecdsa_signature,
    secp256k1_ecdsa_verify, secp256k1_pubkey,
};
use crate::context::Context;
use crate::scalar::Scalar;

#[derive(Debug)]
/// Errors in ECDSA signature operations
pub enum Error {
    /// Error occurred due to invalid secret key
    InvalidSecretKey,
}

/**
Signature is a wrapper around libsecp256k1's internal secp256k1_pubkey struct.
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
Signature is a wrapper around libsecp256k1's internal secp256k1_ecdsa_signature struct.
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
    pub fn verify(self, ctx: &Context, hash: &[u8], pub_key: &PubKey) -> bool {
        let valid = unsafe {
            secp256k1_ecdsa_verify(ctx.context, &self.signature, hash.as_ptr(), &pub_key.key)
        };
        valid == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    fn signature_generation() {
        let ctx = Context::default();

        // Generate a secret and public key
        let mut rnd = OsRng::default();
        let sec_key = Scalar::random(&mut rnd);
        let pub_key = PubKey::new(&ctx, &sec_key).unwrap();

        // Instead of signing a message directly, must sign a 32-byte hash of it.
        // Here the message is "Hello, world!" hashed usings SHA-256.
        let msg_hash: [u8; 32] = [
            0x31, 0x5F, 0x5B, 0xDB, 0x76, 0xD0, 0x78, 0xC4, 0x3B, 0x8A, 0xC0, 0x06, 0x4E, 0x4A,
            0x01, 0x64, 0x61, 0x2B, 0x1F, 0xCE, 0x77, 0xC8, 0x69, 0x34, 0x5B, 0xFC, 0x94, 0xC7,
            0x58, 0x94, 0xED, 0xD3,
        ];

        // Generate a ECDSA signature
        let sig = Signature::new(&ctx, &msg_hash, &sec_key).unwrap();

        // Verify the generated signature is valid using the msg_hash and corresponding public key
        assert!(sig.verify(&ctx, &msg_hash, &pub_key));
    }
}
