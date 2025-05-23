use bitvec::prelude::*;
use bs58;
use core::{
    cmp::{Eq, PartialEq},
    convert::{From, TryFrom},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, BitXor, Mul, MulAssign, Neg, Sub},
};
use num_traits::{One, Zero};
use rand_core::{CryptoRng, RngCore};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::cmp::Ordering;

use crate::_rename::{
    secp256k1_fe_add, secp256k1_fe_cmp_var, secp256k1_fe_get_b32, secp256k1_fe_inv,
    secp256k1_fe_is_odd, secp256k1_fe_mul, secp256k1_fe_negate, secp256k1_fe_normalize,
    secp256k1_fe_normalize_var, secp256k1_fe_set_b32, secp256k1_fe_set_int,
};
use crate::bindings::secp256k1_fe;

use crate::errors::{Base58Error, ConversionError};

use crate::scalar::Scalar;

/// Field size
pub const P: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2F,
];

#[derive(Debug, Clone)]
/// Errors in field element operations
pub enum Error {
    /// Error converting an element
    Conversion(ConversionError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

#[derive(Copy, Clone, Debug)]
/**
Element is a wrapper around libsecp256k1's internal secp256k1_fe struct.  It provides a field element, which is like a scalar but not necessarily reduced modulo the group order
 */
pub struct Element {
    /// The wrapped libsecp256k1 fe
    pub(crate) fe: secp256k1_fe,
}

impl Element {
    /// Construct a zero field element
    pub fn new() -> Self {
        Self {
            fe: secp256k1_fe { n: [0; 5] },
        }
    }

    /// Construct a random field element
    pub fn random<T: RngCore + CryptoRng>(rng: &mut T) -> Self {
        let mut bytes: [u8; 32] = [0; 32];

        rng.fill_bytes(&mut bytes);

        Self::from(bytes)
    }

    /// Set the fe to the passed integer
    pub fn set_int(&mut self, i: i32) {
        unsafe {
            secp256k1_fe_set_int(&mut self.fe, i);
        }
    }

    /// Return the multiplicative inverse of this fe
    pub fn invert(&self) -> Element {
        let mut r = Element::new();

        unsafe {
            secp256k1_fe_inv(&mut r.fe, &self.fe);
        }

        r
    }

    /// Return a byte array of the fe's data in big endian
    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];

        unsafe {
            secp256k1_fe_get_b32(bytes.as_mut_ptr(), &self.fe);
        }

        bytes
    }

    /// Return true if the field element is odd
    pub fn is_odd(&self) -> bool {
        unsafe { secp256k1_fe_is_odd(&self.fe) == 1 }
    }

    /// Fast exponentiation using the square and multiply algorithm
    pub fn square_and_multiply(x: &Self, n: &Self) -> Self {
        let mut r = Self::one();
        let mut square = *x;
        let bytes = n.to_bytes();

        for i in 0..bytes.len() {
            let bits = bytes[31 - i].view_bits::<Lsb0>();
            for bit in bits {
                if *bit {
                    r *= square;
                }
                square *= square;
            }
        }

        unsafe {
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }

    /// Normalize the elemwnt with constant time guarantees
    pub fn normalize(&self) -> Self {
        let mut r = *self;

        unsafe {
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }

    /// Normalize the elemwnt without constant time guarantees
    pub fn normalize_var(&self) -> Self {
        let mut r = *self;

        unsafe {
            secp256k1_fe_normalize_var(&mut r.fe);
        }

        r
    }
}

impl Default for Element {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", bs58::encode(self.to_bytes()).into_string())
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        unsafe { secp256k1_fe_cmp_var(&self.fe, &other.fe) == 0 }
    }
}

impl Eq for Element {}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { secp256k1_fe_cmp_var(&self.fe, &other.fe) } {
            -1 => Ordering::Less,
            0 => Ordering::Equal,
            1 => Ordering::Greater,
            _ => panic!("secp256k1_fe_cmp_var returned unexpected result"), // Unreachable
        }
    }
}

impl Serialize for Element {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.to_bytes())
    }
}

struct ElementVisitor;

impl<'de> Visitor<'de> for ElementVisitor {
    type Value = Element;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter
            .write_str("an array of bytes which represents field element for the secp256k1 curve")
    }

    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match Self::Value::try_from(value) {
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

impl<'de> Deserialize<'de> for Element {
    fn deserialize<D>(deserializer: D) -> Result<Element, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(ElementVisitor)
    }
}

impl Hash for Element {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.to_bytes()[..]);
    }
}

impl From<Scalar> for Element {
    fn from(x: Scalar) -> Self {
        let bytes = x.to_bytes();
        Self::from(bytes)
    }
}

impl From<i32> for Element {
    fn from(i: i32) -> Self {
        let mut s = Element::new();

        s.set_int(i);

        s
    }
}

impl From<[u8; 32]> for Element {
    fn from(bytes: [u8; 32]) -> Self {
        let mut s = Element::new();
        let p: *const u8 = bytes.as_ptr();

        unsafe {
            secp256k1_fe_set_b32(&mut s.fe, p);
        }

        s
    }
}

impl TryFrom<&[u8]> for Element {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Error> {
        match bytes.len() {
            32 => {
                let mut data = [0u8; 32];

                data.clone_from_slice(bytes);
                Ok(Element::from(data))
            }
            n => Err(Error::Conversion(ConversionError::WrongNumberOfBytes(n))),
        }
    }
}

impl TryFrom<&str> for Element {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Error> {
        match bs58::decode(s).into_vec() {
            Ok(bytes) => Element::try_from(&bytes[..]),
            Err(_e) => Err(Error::Conversion(ConversionError::Base58(
                Base58Error::Decode, //(e),
            ))),
        }
    }
}

impl From<Element> for String {
    fn from(s: Element) -> String {
        bs58::encode(s.to_bytes()).into_string()
    }
}

impl Add for Element {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut r = self;

        unsafe {
            secp256k1_fe_add(&mut r.fe, &other.fe);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl Add<&Element> for &Element {
    type Output = Element;

    fn add(self, other: &Element) -> Element {
        let mut r = *self;

        unsafe {
            secp256k1_fe_add(&mut r.fe, &other.fe);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl Add<&Element> for Element {
    type Output = Element;

    fn add(self, other: &Element) -> Element {
        let mut r = self;

        unsafe {
            secp256k1_fe_add(&mut r.fe, &other.fe);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl Add<Element> for &Element {
    type Output = Element;

    fn add(self, other: Element) -> Element {
        let mut r = *self;

        unsafe {
            secp256k1_fe_add(&mut r.fe, &other.fe);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl AddAssign for Element {
    fn add_assign(&mut self, rhs: Element) {
        unsafe {
            secp256k1_fe_add(&mut self.fe, &rhs.fe);
        }
    }
}

impl AddAssign<&Element> for Element {
    fn add_assign(&mut self, rhs: &Element) {
        unsafe {
            secp256k1_fe_add(&mut self.fe, &rhs.fe);
        }
    }
}

impl Mul for Element {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut r = Element::new();

        unsafe {
            secp256k1_fe_mul(&mut r.fe, &self.fe, &other.fe);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl Mul<&Element> for &Element {
    type Output = Element;

    fn mul(self, other: &Element) -> Element {
        let mut r = Element::new();

        unsafe {
            secp256k1_fe_mul(&mut r.fe, &self.fe, &other.fe);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl Mul<Element> for &Element {
    type Output = Element;

    fn mul(self, other: Element) -> Element {
        let mut r = Element::new();

        unsafe {
            secp256k1_fe_mul(&mut r.fe, &self.fe, &other.fe);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl Mul<&Element> for Element {
    type Output = Element;

    fn mul(self, other: &Element) -> Element {
        let mut r = Element::new();

        unsafe {
            secp256k1_fe_mul(&mut r.fe, &self.fe, &other.fe);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl MulAssign for Element {
    fn mul_assign(&mut self, rhs: Element) {
        unsafe {
            secp256k1_fe_mul(&mut self.fe, &self.fe, &rhs.fe);
            secp256k1_fe_normalize(&mut self.fe);
        }
    }
}

impl MulAssign<&Element> for Element {
    fn mul_assign(&mut self, rhs: &Element) {
        unsafe {
            secp256k1_fe_mul(&mut self.fe, &self.fe, &rhs.fe);
            secp256k1_fe_normalize(&mut self.fe);
        }
    }
}

impl Neg for Element {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = Element::new();

        unsafe {
            secp256k1_fe_negate(&mut r.fe, &self.fe, 1);
            secp256k1_fe_normalize(&mut r.fe);
        }

        r
    }
}

impl Neg for &Element {
    type Output = Element;

    fn neg(self) -> Self::Output {
        let mut r = Element::new();

        unsafe {
            secp256k1_fe_negate(&mut r.fe, &self.fe, 1);
        }

        r
    }
}

impl Sub for Element {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl Sub<&Element> for &Element {
    type Output = Element;

    fn sub(self, other: &Element) -> Element {
        self + &(-other)
    }
}

impl Zero for Element {
    fn zero() -> Self {
        Element::from(0)
    }
    fn is_zero(&self) -> bool {
        self == &Element::zero()
    }
}

impl One for Element {
    fn one() -> Self {
        Element::from(1)
    }
    fn is_one(&self) -> bool {
        self == &Element::one()
    }
}

impl BitXor<Element> for Element {
    type Output = Element;

    fn bitxor(self, rhs: Element) -> Self::Output {
        Element::square_and_multiply(&self, &rhs)
    }
}

impl BitXor<Element> for &Element {
    type Output = Element;

    fn bitxor(self, rhs: Element) -> Self::Output {
        Element::square_and_multiply(self, &rhs)
    }
}

impl BitXor<&Element> for Element {
    type Output = Element;

    fn bitxor(self, rhs: &Element) -> Self::Output {
        Element::square_and_multiply(&self, rhs)
    }
}

impl BitXor<&Element> for &Element {
    type Output = Element;

    fn bitxor(self, rhs: &Element) -> Self::Output {
        Element::square_and_multiply(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    fn from() {
        for x in 0..0xff {
            let s = Element::from(x);
            assert_eq!(s.to_bytes()[31], x as u8);
        }
    }

    #[test]
    fn zero() {
        assert_eq!(Element::zero(), Element::from(0));
    }

    #[test]
    fn one() {
        assert_eq!(Element::one(), Element::from(1));
    }

    #[test]
    fn add() {
        for x in 0..0xff {
            for y in 0..0xff {
                assert_eq!(Element::from(x) + Element::from(y), Element::from(x + y));
            }
        }
    }

    #[test]
    fn sub() {
        for x in 0..0xff {
            for y in 0..x {
                assert_eq!(Element::from(x) - Element::from(y), Element::from(x - y));
            }
        }
    }

    #[test]
    fn mul() {
        for x in 0..0xff {
            for y in 0..0xff {
                assert_eq!(Element::from(x) * Element::from(y), Element::from(x * y));
            }
        }
    }

    #[test]
    fn add_assign() {
        for x in 0..0xff {
            for y in 0..0xff {
                let mut s = Element::from(x);
                s += Element::from(y);
                assert_eq!(s, Element::from(x + y));
            }
        }
    }

    #[test]
    fn mul_assign() {
        for x in 0..0xff {
            for y in 0..0xff {
                let mut s = Element::from(x);
                s *= Element::from(y);
                assert_eq!(s, Element::from(x * y));
            }
        }
    }

    #[test]
    fn invert() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Element::random(&mut rng);

            assert_eq!(x * x.invert(), Element::from(1));
        }
    }

    #[test]
    fn neg() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Element::random(&mut rng);

            assert_eq!(x + (-x), Element::from(0));
        }
    }

    #[test]
    fn is_odd() {
        let even = Element::from(2);
        let odd = Element::from(1);

        assert!(!even.is_odd());
        assert!(odd.is_odd());
    }

    #[test]
    fn pow() {
        let mut rng = OsRng::default();
        let k: i32 = 253;
        let ke = Element::from(k);

        for _ in 0..0xff {
            let x = Element::random(&mut rng);

            let klhs = (0..k).fold(Element::one(), |s, _| s * x);
            assert_eq!(klhs, x ^ ke);
        }
    }

    #[test]
    fn cmp() {
        let left = Element::from(1);
        let right = Element::from(2);

        assert!(left < right);
        assert!(right > left);
    }

    #[test]
    fn sort() {
        let sorted = [1, 2, 3, 4, 5].map(Element::from);
        let mut unsorted = [4, 2, 3, 1, 5].map(Element::from);
        unsorted.sort();

        assert_eq!(unsorted, sorted);
    }

    #[test]
    fn base58() {
        let mut rng = OsRng::default();
        let a = Element::random(&mut rng);
        let s = format!("{}", &a);
        let b = Element::try_from(s.as_str()).unwrap();
        let t: String = a.into();
        let c = Element::try_from(t.as_str()).unwrap();

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(s, t);
    }

    #[test]
    fn custom_serde() {
        let mut rng = OsRng::default();
        let x = Element::random(&mut rng);
        let s = serde_json::to_string(&x).expect("failed to serialize");
        let y = serde_json::from_str(&s).expect("failed to deserialize");

        assert_eq!(x, y);
    }
}
