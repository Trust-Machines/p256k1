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
        bytes[31 - i as usize / 8] >> i % 8 & 1 != 0
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
}
