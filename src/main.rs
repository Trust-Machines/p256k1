use core::convert::From;
use core::cmp::PartialEq;
use core::ops::{
    Add,
};

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod bindings;
//mod scalar;

use crate::bindings::{
    secp256k1_scalar, secp256k1_scalar_add, secp256k1_scalar_set_int,
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
        self.scalar.d == other.scalar.d
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

//use crate::scalar::Scalar;

fn main() {
    let scalar32 = Scalar::from(32);
    let scalar10 = Scalar::from(10);
    let scalar42 = scalar32 + scalar10;

    assert_eq!(scalar42, Scalar::from(42));
    
    println!("Hello, world!");
}
