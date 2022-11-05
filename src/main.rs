use core::convert::From;
use core::cmp::PartialEq;
use core::ops::{
    Add, Mul, Neg, Sub
};

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod bindings;
//mod scalar;

use crate::bindings::{
    secp256k1_scalar, secp256k1_scalar_add, secp256k1_scalar_eq, secp256k1_scalar_mul, secp256k1_scalar_negate, secp256k1_scalar_set_int,
};

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

impl Sub for Scalar {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

//use crate::scalar::Scalar;

fn main() {
    assert_eq!(Scalar::from(32) + Scalar::from(10), Scalar::from(42));
    assert_eq!(Scalar::from(32) * Scalar::from(10), Scalar::from(320));
    assert_eq!(Scalar::from(52) - Scalar::from(10), Scalar::from(42));
    
    println!("Hello, world!");
}
