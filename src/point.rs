use core::{
    convert::From,
    cmp::PartialEq,
    fmt::{
        Debug, Display, Formatter, Result,
    },
    ops::{
        Add, Mul, Neg, Sub,
    },
};

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]

use crate::bindings::{
    SECP256K1_CONTEXT_SIGN, secp256k1_context, secp256k1_fe, /*secp256k1_ge,*/ secp256k1_gej, secp256k1_ecmult, secp256k1_gej_add_var, secp256k1_gej_neg, secp256k1_context_create, 
};

use crate::scalar::Scalar;

pub struct Point {
    pub gej: secp256k1_gej,
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

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Point")
         .field("x", &self.gej.x)
         .field("y", &self.gej.y)
         .finish()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?})", self.gej.x, self.gej.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let p = self - other;

        p.gej.infinity == 1
    }
}

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

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        let mut r = Point::new();

        unsafe {
            let null = 0 as *mut secp256k1_fe;
            secp256k1_gej_add_var(&mut r.gej, &self.gej, &other.gej, null);
        }

        r
    }
}

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

impl Neg for &Point {
    type Output = Point;

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

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        self + &(-other)
    }
}
