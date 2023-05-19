use bs58;
use core::{
    cmp::{Eq, PartialEq},
    convert::{From, TryFrom},
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Sub},
};
use num_traits::{One, Zero};
use rand_core::{CryptoRng, RngCore};

use crate::_rename::{
    secp256k1_ecmult, secp256k1_scalar_add, secp256k1_scalar_eq, secp256k1_scalar_get_b32,
    secp256k1_scalar_inverse, secp256k1_scalar_mul, secp256k1_scalar_negate,
    secp256k1_scalar_set_b32, secp256k1_scalar_set_int,
};
use crate::bindings::secp256k1_scalar;

use crate::point::Point;

use crate::errors::{Base58Error, ConversionError};

#[derive(Debug, Clone)]
/// Errors in scalar operations
pub enum Error {
    /// Error converting a scalar
    Conversion(ConversionError),
}

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
/**
Scalar is a wrapper around libsecp256k1's internal secp256k1_scalar struct.  It provides a scalar modulo the group order.  Storing scalars in this format avoids unnecessary conversions from byte bffers, which provides a significant performance enhancement.
 */
pub struct Scalar {
    /// The wrapped libsecp256k1 scalar
    pub scalar: secp256k1_scalar,
}

impl Scalar {
    /// Construct a zero scalar
    pub fn new() -> Self {
        Self {
            scalar: secp256k1_scalar { d: [0; 4] },
        }
    }

    /// Construct a random scalar
    pub fn random<T: RngCore + CryptoRng>(rng: &mut T) -> Self {
        let mut bytes: [u8; 32] = [0; 32];

        rng.fill_bytes(&mut bytes);

        Scalar::from(bytes)
    }

    /// Set the scalar to the passed integer
    pub fn set_int(&mut self, i: u32) {
        unsafe {
            secp256k1_scalar_set_int(&mut self.scalar, i);
        }
    }

    /// Get a u32 from the last 4 bytes of this scalar
    pub fn get_u32(&self) -> u32 {
        let bytes = self.to_bytes();
        let mut u32_be_bytes = [0u8; 4];
        u32_be_bytes.copy_from_slice(&bytes[28..]);

        u32::from_be_bytes(u32_be_bytes)
    }

    /// Return the multiplicative inverse of this scalar
    pub fn invert(&self) -> Scalar {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_inverse(&mut r.scalar, &self.scalar);
        }

        r
    }

    /// Return a byte array of the scalar's data in big endian
    pub fn to_bytes(&self) -> [u8; 32] {
        let mut bytes = [0u8; 32];

        unsafe {
            secp256k1_scalar_get_b32(bytes.as_mut_ptr(), &self.scalar);
        }

        bytes
    }

    fn log2(bytes: &[u8; 32]) -> u32 {
        for i in 0..32 {
            let v = bytes[i as usize];
            if v != 0 {
                return 256 - i * 8 - v.leading_zeros();
            }
        }
        0
    }

    #[inline]
    fn bit(bytes: &[u8; 32], i: u32) -> bool {
        bytes[31 - i as usize / 8] >> (i % 8) & 1 != 0
    }

    /// Fast exponentiation using the square and multiply algorithm
    pub fn square_and_multiply(mut self, n: &Scalar) -> Scalar {
        // Assumption: A Scalar multiplication is an expensive operation,
        // so we are trying to minimize the number of multiplications.

        if self.is_one() || self.is_zero() {
            return self;
        }

        let bytes = n.to_bytes();
        let log2 = Scalar::log2(&bytes);

        let mut ret = Scalar::one();
        let mut i = 0;
        loop {
            if Scalar::bit(&bytes, i) {
                ret *= self;
            }
            if i == log2 {
                return ret;
            }
            i += 1;
            self *= self;
        }
    }

    /// Fast exponentiation using the square and multiply algorithm
    pub fn square_and_multiply_u64(mut self, mut n: u64) -> Scalar {
        // Assumption: A Scalar multiplication is an expensive operation,
        // so we are trying to minimize the number of multiplications.

        if self.is_one() || self.is_zero() {
            return self;
        }

        let mut ret = Scalar::one();
        loop {
            if n & 1 != 0 {
                ret *= self;
            }
            n >>= 1;
            if n == 0 {
                return ret;
            }
            self *= self;
        }
    }
}

impl Default for Scalar {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", bs58::encode(self.to_bytes()).into_string())
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        unsafe { secp256k1_scalar_eq(&self.scalar, &other.scalar) != 0 }
    }
}

impl Eq for Scalar {}

impl Hash for Scalar {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.to_bytes()[..]);
    }
}

impl From<u32> for Scalar {
    fn from(i: u32) -> Self {
        let mut s = Scalar::new();

        s.set_int(i);

        s
    }
}

impl From<[u8; 32]> for Scalar {
    fn from(bytes: [u8; 32]) -> Self {
        let mut s = Scalar::new();
        let null = std::ptr::null_mut::<::std::os::raw::c_int>();
        let p: *const u8 = bytes.as_ptr();

        unsafe {
            secp256k1_scalar_set_b32(&mut s.scalar, p, null);
        }

        s
    }
}

impl TryFrom<&[u8]> for Scalar {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Error> {
        match bytes.len() {
            32 => {
                let mut data = [0u8; 32];

                data.clone_from_slice(bytes);
                Ok(Scalar::from(data))
            }
            n => Err(Error::Conversion(ConversionError::WrongNumberOfBytes(n))),
        }
    }
}

impl TryFrom<&str> for Scalar {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Error> {
        match bs58::decode(s).into_vec() {
            Ok(bytes) => Scalar::try_from(&bytes[..]),
            Err(e) => Err(Error::Conversion(ConversionError::Base58(
                Base58Error::Decode(e),
            ))),
        }
    }
}

impl From<Scalar> for String {
    fn from(s: Scalar) -> String {
        bs58::encode(s.to_bytes()).into_string()
    }
}

impl Add for Scalar {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_add(&mut r.scalar, &self.scalar, &other.scalar);
        }

        r
    }
}

impl Add<&Scalar> for &Scalar {
    type Output = Scalar;

    fn add(self, other: &Scalar) -> Scalar {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_add(&mut r.scalar, &self.scalar, &other.scalar);
        }

        r
    }
}

impl Add<&Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: &Scalar) -> Scalar {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_add(&mut r.scalar, &self.scalar, &other.scalar);
        }

        r
    }
}

impl Add<Scalar> for &Scalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Scalar {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_add(&mut r.scalar, &self.scalar, &other.scalar);
        }

        r
    }
}

impl AddAssign for Scalar {
    fn add_assign(&mut self, rhs: Scalar) {
        unsafe {
            secp256k1_scalar_add(&mut self.scalar, &self.scalar, &rhs.scalar);
        }
    }
}

impl AddAssign<&Scalar> for Scalar {
    fn add_assign(&mut self, rhs: &Scalar) {
        unsafe {
            secp256k1_scalar_add(&mut self.scalar, &self.scalar, &rhs.scalar);
        }
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_mul(&mut r.scalar, &self.scalar, &other.scalar);
        }

        r
    }
}

impl Mul<&Scalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, other: &Scalar) -> Scalar {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_mul(&mut r.scalar, &self.scalar, &other.scalar);
        }

        r
    }
}

impl Mul<Scalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Scalar {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_mul(&mut r.scalar, &self.scalar, &other.scalar);
        }

        r
    }
}

impl Mul<&Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: &Scalar) -> Scalar {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_mul(&mut r.scalar, &self.scalar, &other.scalar);
        }

        r
    }
}

impl MulAssign for Scalar {
    fn mul_assign(&mut self, rhs: Scalar) {
        unsafe {
            secp256k1_scalar_mul(&mut self.scalar, &self.scalar, &rhs.scalar);
        }
    }
}

impl MulAssign<&Scalar> for Scalar {
    fn mul_assign(&mut self, rhs: &Scalar) {
        unsafe {
            secp256k1_scalar_mul(&mut self.scalar, &self.scalar, &rhs.scalar);
        }
    }
}

impl Mul<Point> for Scalar {
    type Output = Point;

    fn mul(self, p: Point) -> Self::Output {
        let mut r = Point::new();
        let zero = Scalar::from(0);

        unsafe {
            secp256k1_ecmult(&mut r.gej, &p.gej, &self.scalar, &zero.scalar);
        }

        r
    }
}

impl Mul<&Point> for &Scalar {
    type Output = Point;

    fn mul(self, p: &Point) -> Self::Output {
        let mut r = Point::new();
        let zero = Scalar::from(0);

        unsafe {
            secp256k1_ecmult(&mut r.gej, &p.gej, &self.scalar, &zero.scalar);
        }

        r
    }
}

impl Mul<Point> for &Scalar {
    type Output = Point;

    fn mul(self, p: Point) -> Self::Output {
        let mut r = Point::new();
        let zero = Scalar::from(0);

        unsafe {
            secp256k1_ecmult(&mut r.gej, &p.gej, &self.scalar, &zero.scalar);
        }

        r
    }
}

impl Mul<&Point> for Scalar {
    type Output = Point;

    fn mul(self, p: &Point) -> Self::Output {
        let mut r = Point::new();
        let zero = Scalar::from(0);

        unsafe {
            secp256k1_ecmult(&mut r.gej, &p.gej, &self.scalar, &zero.scalar);
        }

        r
    }
}

impl Div<Scalar> for Scalar {
    type Output = Scalar;
    fn div(self, q: Scalar) -> Self::Output {
        let q1 = q.invert();
        self * q1
    }
}

impl Div<&Scalar> for &Scalar {
    type Output = Scalar;
    fn div(self, q: &Scalar) -> Self::Output {
        let q1 = q.invert();
        self * q1
    }
}

impl Div<Scalar> for &Scalar {
    type Output = Scalar;
    fn div(self, q: Scalar) -> Self::Output {
        let q1 = q.invert();
        self * q1
    }
}

impl Div<&Scalar> for Scalar {
    type Output = Scalar;
    fn div(self, q: &Scalar) -> Self::Output {
        let q1 = q.invert();
        self * q1
    }
}

impl DivAssign<Scalar> for Scalar {
    fn div_assign(&mut self, q: Scalar) {
        let q1 = q.invert();
        *self *= q1;
    }
}

impl DivAssign<&Scalar> for Scalar {
    fn div_assign(&mut self, q: &Scalar) {
        let q1 = q.invert();
        *self *= q1;
    }
}

impl Neg for Scalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_negate(&mut r.scalar, &self.scalar);
        }

        r
    }
}

impl Neg for &Scalar {
    type Output = Scalar;

    fn neg(self) -> Self::Output {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_negate(&mut r.scalar, &self.scalar);
        }

        r
    }
}

impl Sub for Scalar {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

impl Sub<&Scalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, other: &Scalar) -> Scalar {
        self + &(-other)
    }
}

impl Zero for Scalar {
    fn zero() -> Self {
        Scalar::from(0)
    }
    fn is_zero(&self) -> bool {
        self == &Scalar::zero()
    }
}

impl One for Scalar {
    fn one() -> Self {
        Scalar::from(1)
    }
    fn is_one(&self) -> bool {
        self == &Scalar::one()
    }
}

impl BitXor<usize> for Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: usize) -> Self::Output {
        self.square_and_multiply_u64(rhs as u64)
    }
}

impl BitXor<usize> for &Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: usize) -> Self::Output {
        self.square_and_multiply_u64(rhs as u64)
    }
}

impl BitXor<u32> for Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: u32) -> Self::Output {
        self.square_and_multiply_u64(rhs as u64)
    }
}

impl BitXor<u32> for &Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: u32) -> Self::Output {
        self.square_and_multiply_u64(rhs as u64)
    }
}

impl BitXor<Scalar> for Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: Scalar) -> Self::Output {
        self.square_and_multiply(&rhs)
    }
}

impl BitXor<Scalar> for &Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: Scalar) -> Self::Output {
        self.square_and_multiply(&rhs)
    }
}

impl BitXor<&Scalar> for Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: &Scalar) -> Self::Output {
        self.square_and_multiply(rhs)
    }
}

impl BitXor<&Scalar> for &Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: &Scalar) -> Self::Output {
        self.square_and_multiply(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    fn from() {
        for x in 0..0xff {
            let s = Scalar::from(x);
            assert_eq!(s.to_bytes()[31], x as u8);
        }
    }

    #[test]
    fn get_u32() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = rng.next_u32();
            let s = Scalar::from(x);
            assert_eq!(x, s.get_u32());
        }
    }

    #[test]
    fn zero() {
        assert_eq!(Scalar::zero(), Scalar::from(0));
    }

    #[test]
    fn one() {
        assert_eq!(Scalar::one(), Scalar::from(1));
    }

    #[test]
    fn add() {
        for x in 0..0xff {
            for y in 0..0xff {
                assert_eq!(Scalar::from(x) + Scalar::from(y), Scalar::from(x + y));
            }
        }
    }

    #[test]
    fn sub() {
        for x in 0..0xff {
            for y in 0..x {
                assert_eq!(Scalar::from(x) - Scalar::from(y), Scalar::from(x - y));
            }
        }
    }

    #[test]
    fn mul() {
        for x in 0..0xff {
            for y in 0..0xff {
                assert_eq!(Scalar::from(x) * Scalar::from(y), Scalar::from(x * y));
            }
        }
    }

    #[test]
    fn div() {
        for x in 0..0xff {
            for y in 1..x {
                if x % y == 0 {
                    assert_eq!(Scalar::from(x) / Scalar::from(y), Scalar::from(x / y));
                }
            }
        }
    }

    #[test]
    fn add_assign() {
        for x in 0..0xff {
            for y in 0..0xff {
                let mut s = Scalar::from(x);
                s += Scalar::from(y);
                assert_eq!(s, Scalar::from(x + y));
            }
        }
    }

    #[test]
    fn mul_assign() {
        for x in 0..0xff {
            for y in 0..0xff {
                let mut s = Scalar::from(x);
                s *= Scalar::from(y);
                assert_eq!(s, Scalar::from(x * y));
            }
        }
    }

    #[test]
    fn div_assign() {
        for x in 0..0xff {
            for y in 1..x {
                if x % y == 0 {
                    let mut s = Scalar::from(x);
                    s /= Scalar::from(y);
                    assert_eq!(s, Scalar::from(x / y));
                }
            }
        }
    }

    #[test]
    fn invert() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);

            assert_eq!(x * x.invert(), Scalar::from(1));
        }
    }

    #[test]
    fn neg() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);

            assert_eq!(x + (-x), Scalar::from(0));
        }
    }

    #[test]
    fn pow() {
        let mut rng = OsRng::default();
        let i: u32 = 17;
        let j: usize = 64;
        let k: u32 = 253;
        let ks = Scalar::from(k);

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);

            let ilhs = (0..i).fold(Scalar::one(), |s, _| s * x);
            assert_eq!(ilhs, x ^ i);
            let jlhs = (0..j).fold(Scalar::one(), |s, _| s * x);
            assert_eq!(jlhs, x ^ j);
            let klhs = (0..k).fold(Scalar::one(), |s, _| s * x);
            assert_eq!(klhs, x ^ ks);
        }
    }

    #[test]
    fn base58() {
        let mut rng = OsRng::default();
        let a = Scalar::random(&mut rng);
        let s = format!("{}", &a);
        let b = Scalar::try_from(s.as_str()).unwrap();
        let t: String = a.into();
        let c = Scalar::try_from(t.as_str()).unwrap();

        assert_eq!(a, b);
        assert_eq!(a, c);
        assert_eq!(s, t);
    }

    fn from_be_bytes(bytes: &[u8; 32]) -> Scalar {
        let mut ret = Scalar::new();
        let mut overflow = 0;
        unsafe {
            secp256k1_scalar_set_b32(&mut ret.scalar, bytes.as_ptr(), &mut overflow);
        }
        ret
    }

    fn from_be_u128(hi: u128, lo: u128) -> Scalar {
        let mut bytes = [0u8; 32];
        bytes[..16].clone_from_slice(&hi.to_be_bytes());
        bytes[16..].clone_from_slice(&lo.to_be_bytes());
        from_be_bytes(&bytes)
    }

    #[test]
    fn square_and_multiply_test() {
        let f = |v: u32, e: u32, hi, lo| {
            let sv = Scalar::from(v);
            let se = Scalar::from(e);
            let r = sv.square_and_multiply(&se);
            let r64 = sv.square_and_multiply_u64(e as u64);
            assert_eq!(r, r64);
            assert_eq!(r, from_be_u128(hi, lo));
        };
        f(0, 1, 0, 0);
        f(2, 2, 0, 4);
        f(3, 3, 0, 0x1b);
        f(4, 4, 0, 0x0100);
        f(5, 5, 0, 0x0c35);
        f(6, 6, 0, 0xb640);
        f(7, 7, 0, 0x0c90f7);
        f(8, 8, 0, 0x1000000);
        f(9, 9, 0, 0x17179149);
        f(0xA, 0xA, 0, 0x2540be400);
        f(0xB, 0xB, 0, 0x426de69953);
        f(0xC, 0xC, 0, 0x81bf1000000);
        f(0xD, 0xD, 0, 0x113769b23c5fd);
        f(0xE, 0xE, 0, 0x277a4fb3944000);
        f(0xF, 0xF, 0, 0x613b62c597707ef);
        f(0x10, 0x10, 0, 0x10000000000000000);
        f(0x11, 0x11, 0, 0x2cd843cb4764370911);
        f(0x12, 0x12, 0, 0x854f91a2e471b440000);
        f(0x13, 0x13, 0, 0x1a2f2582d1279edb90b8b);
        f(0x14, 0x14, 0, 0x56bc75e2d6310000000000);
        f(0x15, 0x15, 0, 0x12e0dfece9da1d2a34567945);
        f(0x16, 0x16, 0, 0x44f36854f4522be143a400000);
        f(0x17, 0x17, 0, 0x1078c6e4f7d75450b1fb3ec6ae7);
        f(0x18, 0x18, 0, 0x41c21cb8e1000000000000000000);
        f(0x19, 0x19, 0, 0x111b0ec57e6499a1f4b1014d3f6d59);
        f(0x1A, 0x1A, 0, 0x4a1a02108f971a7f069357024000000);
        f(0x1B, 0x1B, 0x1, 0x4d98d5cea149e834b6bf0c69d56d7cc3);
        f(0x1C, 0x1C, 0x61, 0x67eb8c7252078275a100000000000000);
        f(0x1D, 0x1D, 0x1d79, 0xc05d04235e8807c34cbc36a8b48a4c0d);
        f(0x1E, 0x1E, 0x93b83, 0xa81a7cbba03c2241239c3c4840000000);
        f(0x1F, 0x1F, 0x2fd6882, 0x68dffc136e010737bf943f5988303fdf);
        f(0x20, 0x20, 0x100000000, 0x00000000000000000000000000000000);
        f(0x21, 0x21, 0x5857366dce, 0x0162cb5ddcd1bf0fc7c03a6438304421);
        f(
            0x22,
            0x22,
            0x1f6c3801661b,
            0xd342fb2dd4242160b680cc8400000000,
        );
        f(
            0x23,
            0x23,
            0xb82be3e380ccd,
            0x769982ea6932282974ac767524dc12fb,
        );
        f(
            0x24,
            0x24,
            0x456bc60e76c111e,
            0x679735c929f6a1000000000000000000,
        );
        f(
            0x25,
            0x25,
            0x1ae78c038561352ae,
            0x951ee7b7b012ea77beec149af4096455,
        );
        f(
            0x26,
            0x26,
            0xab67135780fb07de43,
            0xcb7a528e8aa6a1c35acf5e4000000000,
        );
        f(
            0x27,
            0x27,
            0x4611711e18402fcba1e0,
            0xf25f47e1499550e185ef95055e658cd7,
        );
        f(
            0x28,
            0x28,
            0x1d6329f1c35ca4bfabb9f5,
            0x61000000000000000000000000000000,
        );
        f(
            0x29,
            0x29,
            0xca32f2ece8b14b98f6f9690,
            0x356201fde2f9d27da8c738877dc89369,
        );
        f(
            0x2A,
            0x2A,
            0x5919417cd6a11dbdf2f413657,
            0xbbc03bc842ac76ef3932640000000000,
        );
        f(
            0x2B,
            0x2B,
            0x28350f997e1099d39257c99472a,
            0x5022492baddad88dc77fef641843f433,
        );
        f(
            0x2C,
            0x2C,
            0x129236dc5d989e02fc561a2730656,
            0x80110274110000000000000000000000,
        );
        f(
            0x2D,
            0x2D,
            0x8c6627f525d504a78b19877bbf5063,
            0x9983d78ee5d721142426d07d99cae81d,
        );
        f(
            0x2E,
            0x2E,
            0x43d475ec0a902527f4a010f3dde187d9,
            0xf3088a5b0d719423071c400000000000,
        );
        // f(0x2F, 0x2F, 0x7d8b59f36c57a090ac82149b90821a83, 0xecd5c22a0b6f544e1d493f2c647fd7cf);
        // f(0x89AB_CDEF, 0x0123_4567, 0x8635841270a585219fb4300537524776, 0x349160fdb7214cd245a5e3e0d9e7168f);
    }
}
