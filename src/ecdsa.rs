use crate::bindings::{
    secp256k1_ec_pubkey_create, secp256k1_ecdsa_sign, secp256k1_ecdsa_signature,
    secp256k1_ecdsa_verify, secp256k1_pubkey,
};
use crate::context::Context;
use crate::scalar::Scalar;

#[derive(Debug)]
/// Errors in ECDSA signature operations
pub enum Error {
    /// Error occurred signing ECDSA Signature
    SigningFailed,
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
    pub sig: secp256k1_ecdsa_signature,
}

impl Signature {
    /// Construct an ECDSA signature
    pub fn new(ctx: &Context, secret_key: &Scalar, hash: &[u8]) -> Result<Self, Error> {
        let mut signature = Self {
            sig: secp256k1_ecdsa_signature { data: [0; 64] },
        };
        let null = std::ptr::null_mut::<::std::os::raw::c_void>();

        let res = unsafe {
            secp256k1_ecdsa_sign(
                ctx.context,
                &mut signature.sig,
                hash.as_ptr(),
                secret_key.to_bytes().as_ptr(),
                None,
                null,
            )
        };
        if res == 0 {
            return Err(Error::SigningFailed);
        }
        Ok(signature)
    }

    /// Verify an ECDSA signature
    pub fn verify(self, ctx: &Context, pub_key: &PubKey, hash: &[u8]) -> bool {
        let valid =
            unsafe { secp256k1_ecdsa_verify(ctx.context, &self.sig, hash.as_ptr(), &pub_key.key) };
        valid == 1
    }
}
