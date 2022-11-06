use core::{mem, slice};
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
    SECP256K1_CONTEXT_SIGN, secp256k1_context, secp256k1_fe, secp256k1_ge, secp256k1_gej, secp256k1_ecmult, secp256k1_gej_add_var, secp256k1_gej_neg, secp256k1_scalar, secp256k1_context_create, secp256k1_scalar_add, secp256k1_scalar_eq, secp256k1_scalar_mul, secp256k1_scalar_negate, secp256k1_scalar_set_int,
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

    pub fn as_bytes(&self) -> &[u8] {
        let up: *const u64 = self.scalar.d.as_ptr();
        let bp: *const u8 = up as *const u8;
        let bs: &[u8] = unsafe { slice::from_raw_parts(bp, mem::size_of::<u64>() * 4) };

        bs
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

struct Point {
    gej: secp256k1_gej,
}

#[allow(dead_code)]
impl Point {
    pub fn new() -> Self {
        Self {
            gej: secp256k1_gej {
                x: secp256k1_fe {
                    n: [0; 5],
                },
                y: secp256k1_fe {
                    n: [0; 5],
                },
                z: secp256k1_fe {
                    n: [0; 5],
                },
                infinity: 1,
            },
        }
    }

    pub fn ctx() -> *const secp256k1_context {
        unsafe {
            secp256k1_context_create(SECP256K1_CONTEXT_SIGN)
        }
    }
}

/*
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self - other == Point::new()
    }
}
*/

impl From<Scalar> for Point {
    fn from(x: Scalar) -> Self {
        let mut r = Point::new();
        //let null_scalar = 0 as *const secp256k1_scalar;
        //let null_gej = 0 as *const secp256k1_gej;
        let one = Scalar::from(1);
        let p = Point::new();
        
        unsafe {
            //secp256k1_ecmult_gen(&ctx()->ecmult_gen_ctx, &m_data.obj, &r.m_data.obj);
            //secp256k1_ecmult(&mut r.gej, null_gej, null_scalar, &one.scalar);
            secp256k1_ecmult(&mut r.gej, &p.gej, &one.scalar, &x.scalar);
        }

        r
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut r = Point::new();

        unsafe {
            let null = 0 as *mut secp256k1_fe;
            secp256k1_gej_add_var(&mut r.gej, &self.gej, &other.gej, null);
        }

        r
    }
}

/*
impl Add<&Point> for &Point {
    type Output = Self;

    fn add(&self, other: &Self) -> Self {
        let mut r = Point::new();

        unsafe {
            let null = 0 as *mut secp256k1_fe;
            secp256k1_gej_add_var(&mut r.gej, &self.gej, &other.gej, null);
        }

        r
    }
}
*/

impl Mul<Scalar> for Point {
    type Output = Self;

    fn mul(self, x: Scalar) -> Self {
        let mut r = Point::new();
        let zero = Scalar::from(0);

        unsafe {
            //secp256k1_ecmult_gen(&ctx()->ecmult_gen_ctx, &m_data.obj, &r.m_data.obj);
            secp256k1_ecmult(&mut r.gej, &self.gej, &x.scalar, &zero.scalar);
        }

        r
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut r = Point::new();

        unsafe {
            secp256k1_gej_neg(&mut r.gej, &self.gej);
        }

        r
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + (-other)
    }
}

//use crate::gej::Scalar;

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(non_upper_case_globals)]
fn main() {
    let G: Point = Point::from(Scalar::from(1));

    assert_eq!(Scalar::from(32) + Scalar::from(10), Scalar::from(42));
    assert_eq!(Scalar::from(32) * Scalar::from(10), Scalar::from(320));
    assert_eq!(Scalar::from(52) - Scalar::from(10), Scalar::from(42));

    println!("Scalar(42) bytes {:?}", Scalar::from(42).as_bytes());
}
