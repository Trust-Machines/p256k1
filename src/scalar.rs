use core::{
    cmp::{Eq, PartialEq},
    convert::From,
    fmt::{Debug, Display, Formatter, Result},
    hash::{Hash, Hasher},
    mem,
    ops::{Add, AddAssign, BitXor, Div, DivAssign, Mul, MulAssign, Neg, Sub},
    slice,
};
use num_traits::{One, Zero};
use rand_core::{CryptoRng, RngCore};

use crate::bindings::{
    secp256k1_ecmult, secp256k1_scalar, secp256k1_scalar_add, secp256k1_scalar_eq,
    secp256k1_scalar_inverse, secp256k1_scalar_mul, secp256k1_scalar_negate,
    secp256k1_scalar_set_b32, secp256k1_scalar_set_int,
};

use crate::point::Point;

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Scalar {
    pub scalar: secp256k1_scalar,
}

impl Scalar {
    pub fn new() -> Self {
        Self {
            scalar: secp256k1_scalar { d: [0; 4] },
        }
    }

    pub fn random<T: RngCore + CryptoRng>(rng: &mut T) -> Self {
        let mut bytes: [u8; 32] = [0; 32];

        rng.fill_bytes(&mut bytes);

        Scalar::from(bytes)
    }

    pub fn set_int(&mut self, i: u32) {
        unsafe {
            secp256k1_scalar_set_int(&mut self.scalar, i);
        }
    }

    pub fn invert(&self) -> Scalar {
        let mut r = Scalar::new();

        unsafe {
            secp256k1_scalar_inverse(&mut r.scalar, &self.scalar);
        }

        r
    }

    pub fn as_bytes(&self) -> &[u8] {
        let up: *const u64 = self.scalar.d.as_ptr();
        let bp: *const u8 = up as *const u8;
        let bs: &[u8] = unsafe { slice::from_raw_parts(bp, mem::size_of::<u64>() * 4) };

        bs
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", hex::encode(self.as_bytes()))
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
        state.write(self.as_bytes());
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
        let null = 0 as *mut ::std::os::raw::c_int;
        let p: *const u8 = bytes.as_ptr();

        unsafe {
            secp256k1_scalar_set_b32(&mut s.scalar, p, null);
        }

        s
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
        *self = self.clone() * q1;
    }
}

impl DivAssign<&Scalar> for Scalar {
    fn div_assign(&mut self, q: &Scalar) {
        let q1 = q.invert();
        *self = self.clone() * q1;
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
        let mut ret = Scalar::one();

        for _ in 0..rhs {
            ret *= self;
        }

        ret
    }
}

impl BitXor<usize> for &Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: usize) -> Self::Output {
        let mut ret = Scalar::one();

        for _ in 0..rhs {
            ret *= self;
        }

        ret
    }
}

impl BitXor<u32> for Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: u32) -> Self::Output {
        let mut ret = Scalar::one();

        for _ in 0..rhs {
            ret *= self;
        }

        ret
    }
}

impl BitXor<u32> for &Scalar {
    type Output = Scalar;

    fn bitxor(self, rhs: u32) -> Self::Output {
        let mut ret = Scalar::one();

        for _ in 0..rhs {
            ret *= self;
        }

        ret
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
            assert_eq!(s.as_bytes()[0], x as u8);
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
        let i: usize = 4;
        let j: u32 = 5;

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);

            assert_eq!(x * x * x * x, x ^ i);
            assert_eq!(x * x * x * x * x, x ^ j);
        }
    }
}
