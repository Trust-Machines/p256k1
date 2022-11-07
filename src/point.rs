use core::{
    convert::From,
    cmp::PartialEq,
    fmt::{
        Debug, Display, Formatter, Result,
    },
    ops::{
        Add, AddAssign, Mul, Neg, Sub,
    },
    mem, slice
};
use::num_traits::Zero;

use crate::bindings::{
    SECP256K1_CONTEXT_SIGN, SECP256K1_TAG_PUBKEY_ODD, SECP256K1_TAG_PUBKEY_EVEN, secp256k1_context, secp256k1_fe, secp256k1_ge, secp256k1_gej, secp256k1_ecmult, secp256k1_gej_add_var, secp256k1_gej_neg, secp256k1_context_create, secp256k1_ge_set_gej, secp256k1_ge_set_xo_var, secp256k1_gej_set_ge, secp256k1_fe_normalize_var, secp256k1_fe_is_odd, secp256k1_fe_get_b32, secp256k1_fe_set_b32,
};

use crate::scalar::Scalar;

#[derive(Clone)]
pub struct Point {
    pub gej: secp256k1_gej,
}

#[allow(dead_code)]
impl Point {
    pub fn new() -> Self {
        Self::identity()
    }

    pub fn identity() -> Self {
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

    #[allow(non_snake_case)]
    pub fn G() -> Point {
        Point::from(Scalar::from(1))
    }

    pub fn compress(&mut self) -> Compressed {
        unsafe {
            let mut ge = secp256k1_ge{
                x: secp256k1_fe {
                    n: [0; 5],
                },
                y: secp256k1_fe {
                    n: [0; 5],
                },
                infinity: 0,
            };

            secp256k1_ge_set_gej(&mut ge, &mut self.gej);
	        secp256k1_fe_normalize_var(&mut ge.x);
            secp256k1_fe_normalize_var(&mut ge.y);

            let mut c = Compressed {
                data: [0; 33],
            };
            
            c.data[0] = if secp256k1_fe_is_odd(&ge.y) == 1 {
                SECP256K1_TAG_PUBKEY_ODD.try_into().unwrap()
            } else {
                SECP256K1_TAG_PUBKEY_EVEN.try_into().unwrap()
            };
            
	        secp256k1_fe_get_b32(&mut c.data[1], &ge.x);

            c
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Point")
         .field("x", &self.gej.x)
         .field("y", &self.gej.y)
         .field("z", &self.gej.z)
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
        let one = Scalar::from(1);
        let p = Point::new();
        
        unsafe {
            secp256k1_ecmult(&mut r.gej, &p.gej, &one.scalar, &x.scalar);
        }

        r
    }
}

impl From<&Scalar> for Point {
    fn from(x: &Scalar) -> Self {
        let mut r = Point::new();
        let one = Scalar::from(1);
        let p = Point::new();
        
        unsafe {
            secp256k1_ecmult(&mut r.gej, &p.gej, &one.scalar, &x.scalar);
        }

        r
    }
}

impl From<Compressed> for Point {
    fn from(c: Compressed) -> Self {
        unsafe {
            let mut y = secp256k1_ge{
                x: secp256k1_fe {
                    n: [0; 5],
                },
                y: secp256k1_fe {
                    n: [0; 5],
                },
                infinity: 1,
            };
            
            let mut x = secp256k1_fe {
                n: [0; 5],
            };

            let _rx = secp256k1_fe_set_b32(&mut x, &c.data[1]);
            let _ry = secp256k1_ge_set_xo_var(&mut y, &x, (c.data[0] as u32 == SECP256K1_TAG_PUBKEY_ODD).try_into().unwrap());

            let mut r = Point::new();
            
            secp256k1_gej_set_ge(&mut r.gej, &y);

            r
        }
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

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        let mut r = Point::new();

        unsafe {
            let null = 0 as *mut secp256k1_fe;
            secp256k1_gej_add_var(&mut r.gej, &self.gej, &other.gej, null);
        }

        r
    }
}

impl Add<&Point> for Point {
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

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        unsafe {
            let null = 0 as *mut secp256k1_fe;
            secp256k1_gej_add_var(&mut self.gej, &self.gej, &other.gej, null);
        }
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, other: &Point) {
        unsafe {
            let null = 0 as *mut secp256k1_fe;
            secp256k1_gej_add_var(&mut self.gej, &self.gej, &other.gej, null);
        }
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

impl Mul<&Scalar> for &Point {
    type Output = Point;

    fn mul(self, x: &Scalar) -> Point {
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

impl Zero for Point {
     fn zero() -> Self {
 	     Point::identity()
     }
     fn is_zero(&self) -> bool {
 	     self == &Point::identity()
     }
}

pub struct Compressed {
    data: [u8; 33],
}

impl Compressed {
    pub fn as_bytes(&self) -> &[u8] {
        let up: *const u8 = self.data.as_ptr();
        let bs: &[u8] = unsafe { slice::from_raw_parts(up, mem::size_of::<u8>() * 33) };

        bs

    }
}

impl From<[u8; 33]> for Compressed {
    fn from(bytes: [u8; 33]) -> Self {
        Self {
            data: bytes,
        }
    }
}
