#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]

use core::{
    convert::From,
    cmp::PartialEq,
    fmt::{
        Debug, Display, Formatter, Result,
    },
    ops::{
        Add, Mul, Neg, Sub,
    },
    mem, slice
};

use crate::bindings::{
    secp256k1_scalar, secp256k1_ecmult, secp256k1_scalar_add, secp256k1_scalar_eq, secp256k1_scalar_mul, secp256k1_scalar_negate, secp256k1_scalar_set_int,
};

use crate::point::Point;

#[derive(Debug)]
pub struct Scalar {
    pub scalar: secp256k1_scalar,
}

impl Scalar {
    pub fn new() -> Self {
        Self {
            scalar: secp256k1_scalar {
                d: [0; 4],
            }
        }
    }
    
    pub fn set_int(&mut self, i: u32) {
        unsafe {
            secp256k1_scalar_set_int(&mut self.scalar, i);
        }
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
        write!(f, "{:?}", self.as_bytes())
    }
}

impl PartialEq for Scalar {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            secp256k1_scalar_eq(&self.scalar, &other.scalar) != 0
        }
    }
}

impl From<u32> for Scalar {
    fn from(i: u32) -> Self {
        let mut s = Scalar::new();

        s.set_int(i);
        
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

impl Mul<Point> for Scalar {
    type Output = Point;

    fn mul(self, p: Point) -> Self::Output {
        let mut r = Point::new();
        let zero = Scalar::from(0);

        unsafe {
            //secp256k1_ecmult_gen(&ctx()->ecmult_gen_ctx, &m_data.obj, &r.m_data.obj);
            secp256k1_ecmult(&mut r.gej, &p.gej, &self.scalar, &zero.scalar);
        }

        r
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

