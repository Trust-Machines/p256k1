use bs58;
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::array::TryFromSliceError;

use crate::_rename::{
    secp256k1_ec_pubkey_create, secp256k1_ec_pubkey_parse, secp256k1_ec_pubkey_serialize,
    secp256k1_keypair_create, secp256k1_keypair_pub, secp256k1_keypair_sec,
    secp256k1_keypair_xonly_pub, secp256k1_xonly_pubkey_from_pubkey, secp256k1_xonly_pubkey_parse,
    secp256k1_xonly_pubkey_serialize,
};
use crate::bindings::{
    secp256k1_keypair, secp256k1_pubkey, secp256k1_xonly_pubkey, SECP256K1_EC_COMPRESSED,
};
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
    /// Error occurred due to invalid xonly public key
    InvalidXOnlyPublicKey,
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

/**
XOnlyPublicKey is a wrapper around libsecp256k1's secp256k1_pubkey struct.
*/
#[derive(Clone, Copy)]
pub struct XOnlyPublicKey {
    /// The wrapped secp256k1_pubkey public key
    pub key: secp256k1_xonly_pubkey,
    /// The parity bit of this key
    pub parity: i32,
}

impl XOnlyPublicKey {
    /// Construct a public key from a given secret key
    pub fn new(sec_key: &Scalar) -> Result<Self, Error> {
        let public_key = PublicKey::new(sec_key)?;
        Self::try_from(&public_key)
    }

    /// Serialize the key to a compressed byte array
    pub fn to_bytes(&self) -> [u8; 32] {
        let ctx = Context::default();
        let mut bytes = [0u8; 32];

        unsafe {
            secp256k1_xonly_pubkey_serialize(ctx.context, bytes.as_mut_ptr(), &self.key);
        }

        bytes
    }
}

impl Debug for XOnlyPublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("XOnlyPublicKey")
            .field("data", &bs58::encode(self.key.data).into_string())
            .finish()
    }
}

impl Display for XOnlyPublicKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", bs58::encode(self.to_bytes()).into_string())
    }
}

impl TryFrom<&str> for XOnlyPublicKey {
    type Error = Error;
    /// Create a pubkey from the passed byte slice
    fn try_from(s: &str) -> Result<Self, self::Error> {
        match bs58::decode(s).into_vec() {
            Ok(bytes) => XOnlyPublicKey::try_from(&bytes[..]),
            Err(e) => Err(Error::Conversion(ConversionError::Base58(
                Base58Error::Decode(e),
            ))),
        }
    }
}

impl TryFrom<&[u8]> for XOnlyPublicKey {
    type Error = Error;
    /// Create a pubkey from the passed byte slice
    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        if input.len() != 32 {
            return Err(Error::InvalidXOnlyPublicKey);
        }
        let mut pubkey = Self {
            key: secp256k1_xonly_pubkey { data: [0; 64] },
            parity: 0,
        };
        let ctx = Context::default();
        unsafe {
            match secp256k1_xonly_pubkey_parse(ctx.context, &mut pubkey.key, input.as_ptr()) {
                1 => Ok(pubkey),
                _ => Err(Error::InvalidXOnlyPublicKey),
            }
        }
    }
}

impl TryFrom<&PublicKey> for XOnlyPublicKey {
    type Error = Error;
    /// Create XOnlyPublicKey from the passed PublicKey
    fn try_from(input: &PublicKey) -> Result<Self, Self::Error> {
        let mut output = Self {
            key: secp256k1_xonly_pubkey { data: [0; 64] },
            parity: 0,
        };
        let ctx = Context::default();
        if unsafe {
            secp256k1_xonly_pubkey_from_pubkey(
                ctx.context,
                &mut output.key,
                &mut output.parity,
                &input.key,
            )
        } == 0
        {
            return Err(Error::InvalidSecretKey);
        }
        Ok(output)
    }
}

/**
KeyPair is a wrapper around libsecp256k1's secp256k1_pubkey struct.
*/
#[derive(Clone, Copy)]
pub struct KeyPair {
    /// The wrapped secp256k1_keypair
    pub key: secp256k1_keypair,
}

impl KeyPair {
    /// Construct a keypair from a given secret key
    pub fn new(sec_key: &Scalar) -> Result<Self, Error> {
        let mut pub_key = Self {
            key: secp256k1_keypair { data: [0; 96] },
        };
        let ctx = Context::default();
        if unsafe {
            secp256k1_keypair_create(ctx.context, &mut pub_key.key, sec_key.to_bytes().as_ptr())
        } == 0
        {
            return Err(Error::InvalidSecretKey);
        }
        Ok(pub_key)
    }
}

impl Debug for KeyPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("KeyPair")
            .field("data", &bs58::encode(self.key.data).into_string())
            .finish()
    }
}

impl From<&KeyPair> for Scalar {
    fn from(kp: &KeyPair) -> Scalar {
        let mut bytes = [0u8; 32];
        let ctx = Context::default();

        unsafe {
            let _ = secp256k1_keypair_sec(ctx.context, bytes.as_mut_ptr(), &kp.key);
        }

        Scalar::from(bytes)
    }
}

impl From<&KeyPair> for PublicKey {
    fn from(kp: &KeyPair) -> PublicKey {
        let mut key = secp256k1_pubkey { data: [0; 64] };
        let ctx = Context::default();

        unsafe {
            let _ = secp256k1_keypair_pub(ctx.context, &mut key, &kp.key);
        }

        PublicKey { key }
    }
}

impl From<&KeyPair> for XOnlyPublicKey {
    fn from(kp: &KeyPair) -> XOnlyPublicKey {
        let mut key = XOnlyPublicKey {
            key: secp256k1_xonly_pubkey { data: [0; 64] },
            parity: 0,
        };
        let ctx = Context::default();

        unsafe {
            let _ =
                secp256k1_keypair_xonly_pub(ctx.context, &mut key.key, &mut key.parity, &kp.key);
        }

        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use rand_core::OsRng;

    #[test]
    fn pubkey_serde() {
        // Generate a secret and public key
        let mut rnd = OsRng;
        let seckey = Scalar::random(&mut rnd);
        let pubkey = PublicKey::new(&seckey).unwrap();

        //Serialize with try_from and deseriailze with to_bytes
        let pubkey2 = PublicKey::try_from(pubkey.to_bytes().as_slice()).unwrap();
        assert_eq!(pubkey2.to_bytes(), pubkey.to_bytes());

        //Serialize again with str
        let pubkey3 = PublicKey::try_from(format!("{}", &pubkey).as_str()).unwrap();
        assert_eq!(pubkey3.to_bytes(), pubkey.to_bytes());
        assert_eq!(pubkey3.to_bytes(), pubkey2.to_bytes());
    }

    #[test]
    fn xonlykey_serde() {
        // Generate a secret and public key
        let mut rnd = OsRng;
        let seckey = Scalar::random(&mut rnd);
        let xopubkey = XOnlyPublicKey::new(&seckey).unwrap();

        //Serialize with try_from and deseriailze with to_bytes
        let xopubkey2 = XOnlyPublicKey::try_from(xopubkey.to_bytes().as_slice()).unwrap();
        assert_eq!(xopubkey2.to_bytes(), xopubkey.to_bytes());

        //Serialize again with str
        let xopubkey3 = XOnlyPublicKey::try_from(format!("{}", &xopubkey).as_str()).unwrap();
        assert_eq!(xopubkey3.to_bytes(), xopubkey.to_bytes());
        assert_eq!(xopubkey3.to_bytes(), xopubkey2.to_bytes());
    }

    #[test]
    fn xonlykey_point() {
        // Generate a secret and public key
        let mut rnd = OsRng;
        let scalar = Scalar::random(&mut rnd);
        let point = Point::from(&scalar);
        let xopubkey = XOnlyPublicKey::try_from(&point.x().to_bytes()[..]).unwrap();
        let xopubkey2 = XOnlyPublicKey::new(&scalar).unwrap();

        assert_eq!(xopubkey.to_bytes(), point.x().to_bytes());
        assert_eq!(xopubkey2.to_bytes(), point.x().to_bytes());
    }

    #[test]
    fn keypair() {
        // Generate a secret and public key
        let mut rnd = OsRng;
        let scalar = Scalar::random(&mut rnd);
        let keypair = KeyPair::new(&scalar).unwrap();
        let point = Point::from(&scalar);
        let xopubkey = XOnlyPublicKey::try_from(&point.x().to_bytes()[..]).unwrap();
        let xopubkey2 = XOnlyPublicKey::new(&scalar).unwrap();
        let xopubkey3 = XOnlyPublicKey::from(&keypair);
        let seckey = Scalar::from(&keypair);
        let pubkey = PublicKey::from(&keypair);
        let pubkey2 = PublicKey::new(&scalar).unwrap();
        let pubkey3 = PublicKey::new(&seckey).unwrap();

        assert_eq!(scalar.to_bytes(), seckey.to_bytes());
        assert_eq!(xopubkey.to_bytes(), point.x().to_bytes());
        assert_eq!(xopubkey2.to_bytes(), point.x().to_bytes());
        assert_eq!(xopubkey3.to_bytes(), point.x().to_bytes());
        assert_eq!(pubkey.to_bytes(), pubkey2.to_bytes());
        assert_eq!(pubkey.to_bytes(), pubkey3.to_bytes());
    }
}
