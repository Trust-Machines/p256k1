use bs58;
use core::{
    cmp::{Eq, PartialEq},
    convert::{From, TryFrom},
    ffi::CStr,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    mem,
    ops::{Add, AddAssign, Mul, Neg, Sub},
    slice,
};
use num_traits::Zero;
use primitive_types::U256;
use std::os::raw::c_void;

use crate::{
    bindings::{
        secp256k1_callback, secp256k1_ecmult_multi_callback, secp256k1_fe, secp256k1_ge,
        secp256k1_gej, secp256k1_scalar, SECP256K1_TAG_PUBKEY_EVEN, SECP256K1_TAG_PUBKEY_ODD,
    },
    errors::{Base58Error, ConversionError},
    group::secp256k1_ge_set_gej,
};

use crate::_rename::{
    secp256k1_ecmult, secp256k1_ecmult_multi_var, secp256k1_fe_get_b32, secp256k1_fe_is_odd,
    secp256k1_fe_normalize_var, secp256k1_fe_set_b32, secp256k1_ge_set_xo_var,
    secp256k1_gej_add_var, secp256k1_gej_neg, secp256k1_gej_set_ge, secp256k1_scratch_space_create,
    secp256k1_scratch_space_destroy,
};

use crate::{context::Context, field, scalar::Scalar};

/// The secp256k1 base point
pub const G: Point = Point {
    gej: secp256k1_gej {
        x: secp256k1_fe {
            n: [
                705178180786072,
                3855836460717471,
                4089131105950716,
                3301581525494108,
                133858670344668,
            ],
        },
        y: secp256k1_fe {
            n: [
                2199641648059576,
                1278080618437060,
                3959378566518708,
                3455034269351872,
                79417610544803,
            ],
        },
        z: secp256k1_fe { n: [1, 0, 0, 0, 0] },
        infinity: 0,
    },
};

/// Group order
pub const N: [u8; 32] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFE,
    0xBA, 0xAE, 0xDC, 0xE6, 0xAF, 0x48, 0xA0, 0x3B, 0xBF, 0xD2, 0x5E, 0x8C, 0xD0, 0x36, 0x41, 0x41,
];

#[derive(Debug, Clone)]
/// Errors in point operations
pub enum Error {
    /// Error doing multi-exponentiation
    MultiMultFailed,
    /// Error decompressing a point
    Conversion(ConversionError),
    /// Error lifting a field element into an even_y point
    LiftFailed,
}

#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
/**
Point is a wrapper around libsecp256k1's internal secp256k1_gej struct.  It provides a point on the secp256k1 curve in Jacobian coordinates.  This allows for extremely fast curve point operations, and avoids expensive conversions from byte buffers.
 */
pub struct Point {
    /// The wrapped libsecp256k1 point
    pub gej: secp256k1_gej,
}

#[no_mangle]
extern "C" fn error_callback(
    text: *const ::std::os::raw::c_char,
    _data: *mut ::std::os::raw::c_void,
) {
    unsafe {
        let c_str: &CStr = CStr::from_ptr(text);
        let s: &str = c_str.to_str().unwrap();
        println!("error_callback({s})");
    }
}

struct ScalarsPoints {
    s: Vec<Scalar>,
    p: Vec<Point>,
}

#[no_mangle]
extern "C" fn ecmult_multi_callback(
    sc: *mut secp256k1_scalar,
    pt: *mut secp256k1_ge,
    idx: usize,
    data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    unsafe {
        let sp: *mut ScalarsPoints = data as *mut ScalarsPoints;

        secp256k1_ge_set_gej(&mut *pt, &(*sp).p[idx].gej);
        *sc = (*sp).s[idx].scalar;
    }

    1
}

#[allow(dead_code)]
impl Point {
    /// Construct a new point
    pub fn new() -> Self {
        Self::identity()
    }

    /// Construct an identity point
    pub fn identity() -> Self {
        Self {
            gej: secp256k1_gej {
                x: secp256k1_fe { n: [0; 5] },
                y: secp256k1_fe { n: [0; 5] },
                z: secp256k1_fe { n: [0; 5] },
                infinity: 1,
            },
        }
    }

    #[allow(non_snake_case)]
    /// Return the base point
    pub fn G() -> Point {
        Point::from(Scalar::from(1))
    }

    /// Convert the point into compressed binary format
    pub fn compress(&self) -> Compressed {
        unsafe {
            let mut ge = secp256k1_ge {
                x: secp256k1_fe { n: [0; 5] },
                y: secp256k1_fe { n: [0; 5] },
                infinity: 0,
            };

            secp256k1_ge_set_gej(&mut ge, &self.gej);
            secp256k1_fe_normalize_var(&mut ge.x);
            secp256k1_fe_normalize_var(&mut ge.y);

            let mut c = Compressed { data: [0; 33] };

            c.data[0] = if secp256k1_fe_is_odd(&ge.y) == 1 {
                SECP256K1_TAG_PUBKEY_ODD.try_into().unwrap()
            } else {
                SECP256K1_TAG_PUBKEY_EVEN.try_into().unwrap()
            };

            secp256k1_fe_get_b32(&mut c.data[1], &ge.x);

            c
        }
    }

    /// Perform a multi-exponentiation operation on the passed scalars and points, using the Pipperger algorithm
    pub fn multimult(scalars: Vec<Scalar>, points: Vec<Point>) -> Result<Point, Error> {
        let mut r = Point::new();
        let n = scalars.len();
        let mut sp = ScalarsPoints {
            s: scalars,
            p: points,
        };
        let sp_ptr: *mut c_void = &mut sp as *mut _ as *mut c_void;
        let error_callback_data = [0u8; 32];
        let error_callback_data_ptr: *const c_void =
            &error_callback_data as *const _ as *const c_void;
        let multi_error_callback = secp256k1_callback {
            fn_: Some(error_callback),
            data: error_callback_data_ptr,
        };

        let zero = Scalar::zero();
        let ctx = Context::default();
        let multi_callback: secp256k1_ecmult_multi_callback = Some(ecmult_multi_callback);

        unsafe {
            let scratch = secp256k1_scratch_space_create(ctx.context, 1048576);
            let i = secp256k1_ecmult_multi_var(
                &multi_error_callback,
                scratch,
                &mut r.gej,
                &zero.scalar,
                multi_callback,
                sp_ptr,
                n,
            );
            secp256k1_scratch_space_destroy(ctx.context, scratch);
            if i == 0 {
                return Err(Error::MultiMultFailed);
            }
        }

        Ok(r)
    }

    /// Return true if the y coordinate of this point is even
    pub fn has_even_y(&self) -> bool {
        let mut ge = secp256k1_ge {
            x: secp256k1_fe { n: [0; 5] },
            y: secp256k1_fe { n: [0; 5] },
            infinity: 0,
        };

        unsafe {
            secp256k1_ge_set_gej(&mut ge, &self.gej);
            secp256k1_fe_is_odd(&ge.y) == 0
        }
    }

    /// Return the x coord of this point as a normalized field element
    pub fn x(&self) -> field::Element {
        let mut ge = secp256k1_ge {
            x: secp256k1_fe { n: [0; 5] },
            y: secp256k1_fe { n: [0; 5] },
            infinity: 0,
        };

        unsafe {
            secp256k1_ge_set_gej(&mut ge, &self.gej);
            secp256k1_fe_normalize_var(&mut ge.x);
            field::Element { fe: ge.x }
        }
    }

    /// Return the y coord of this point as a normalized field element
    pub fn y(&self) -> field::Element {
        let mut ge = secp256k1_ge {
            x: secp256k1_fe { n: [0; 5] },
            y: secp256k1_fe { n: [0; 5] },
            infinity: 0,
        };

        unsafe {
            secp256k1_ge_set_gej(&mut ge, &self.gej);
            secp256k1_fe_normalize_var(&mut ge.y);
            field::Element { fe: ge.y }
        }
    }

    /// return the point P for which x(P) = x and has_even_y(P), or fails if x is greater than p-1 or no such point exists
    pub fn lift_x(x: &field::Element) -> Result<Point, Error> {
        let fp = field::Element::from(field::P);
        let p = U256::from_big_endian(&field::P);
        let p14 = (p + 1) / 4;
        let mut p14_bytes = [0u8; 32];

        p14.to_big_endian(&mut p14_bytes);

        let fp14 = field::Element::from(p14_bytes);
        let c = x * x * x + field::Element::from(7);
        let y = c ^ fp14;

        if c != y * y {
            return Err(Error::LiftFailed);
        }

        let point = Point::from((*x, y));
        if point.has_even_y() {
            Ok(point)
        } else {
            Ok(Point::from((*x, fp - y)))
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point::identity()
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Point")
            .field("x", &self.gej.x)
            .field("y", &self.gej.y)
            .field("z", &self.gej.z)
            .finish()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let c = self.clone().compress();
        write!(f, "{}", bs58::encode(c.data).into_string())
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let p = self - other;

        p.gej.infinity == 1
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(&self.compress().data[..]);
    }
}

impl From<(Scalar, Scalar)> for Point {
    fn from(ss: (Scalar, Scalar)) -> Self {
        let x = field::Element::from(ss.0);
        let y = field::Element::from(ss.0);

        Self::from((x, y))
    }
}

impl From<(field::Element, field::Element)> for Point {
    fn from(ff: (field::Element, field::Element)) -> Self {
        unsafe {
            let ge = secp256k1_ge {
                x: ff.0.fe,
                y: ff.1.fe,
                infinity: 0,
            };

            let mut r = Point::new();

            secp256k1_gej_set_ge(&mut r.gej, &ge);

            r
        }
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

impl TryFrom<&Compressed> for Point {
    type Error = Error;

    fn try_from(c: &Compressed) -> Result<Self, Self::Error> {
        unsafe {
            let mut y = secp256k1_ge {
                x: secp256k1_fe { n: [0; 5] },
                y: secp256k1_fe { n: [0; 5] },
                infinity: 1,
            };

            let mut x = secp256k1_fe { n: [0; 5] };

            let rx = secp256k1_fe_set_b32(&mut x, &c.data[1]);
            if rx == 0 {
                return Err(Error::Conversion(ConversionError::BadFieldElement));
            }

            let ry = secp256k1_ge_set_xo_var(
                &mut y,
                &x,
                (c.data[0] as u32 == SECP256K1_TAG_PUBKEY_ODD)
                    .try_into()
                    .unwrap(),
            );
            if ry == 0 {
                return Err(Error::Conversion(ConversionError::BadGroupElement));
            }

            let mut r = Point::new();

            secp256k1_gej_set_ge(&mut r.gej, &y);

            Ok(r)
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut r = Point::new();

        unsafe {
            let null = std::ptr::null_mut::<secp256k1_fe>();
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
            let null = std::ptr::null_mut::<secp256k1_fe>();
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
            let null = std::ptr::null_mut::<secp256k1_fe>();
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
            let null = std::ptr::null_mut::<secp256k1_fe>();
            secp256k1_gej_add_var(&mut r.gej, &self.gej, &other.gej, null);
        }

        r
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Point) {
        unsafe {
            let null = std::ptr::null_mut::<secp256k1_fe>();
            secp256k1_gej_add_var(&mut self.gej, &self.gej, &other.gej, null);
        }
    }
}

impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, other: &Point) {
        unsafe {
            let null = std::ptr::null_mut::<secp256k1_fe>();
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

/// A Point in compressed binary format
pub struct Compressed {
    /// The raw bytes of the compressed point
    pub data: [u8; 33],
}

impl Compressed {
    /// Return a byte slice of the data
    pub fn as_bytes(&self) -> &[u8] {
        let up: *const u8 = self.data.as_ptr();
        #[allow(clippy::size_of_in_element_count)]
        let bs: &[u8] = unsafe { slice::from_raw_parts(up, mem::size_of::<u8>() * 33) };

        bs
    }
}

impl From<[u8; 33]> for Compressed {
    fn from(bytes: [u8; 33]) -> Self {
        Self { data: bytes }
    }
}

impl From<Compressed> for String {
    fn from(c: Compressed) -> String {
        bs58::encode(c.data).into_string()
    }
}

impl TryFrom<&[u8]> for Compressed {
    type Error = Error;
    fn try_from(bytes: &[u8]) -> Result<Self, Error> {
        match bytes.len() {
            33 => {
                let mut r = Compressed { data: [0; 33] };

                r.data.clone_from_slice(bytes);
                Ok(r)
            }
            n => Err(Error::Conversion(ConversionError::WrongNumberOfBytes(n))),
        }
    }
}

impl TryFrom<&str> for Compressed {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self, Error> {
        match bs58::decode(s).into_vec() {
            Ok(bytes) => Compressed::try_from(&bytes[..]),
            Err(e) => Err(Error::Conversion(ConversionError::Base58(
                Base58Error::Decode(e),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::OsRng;

    #[test]
    fn from() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);
            let p = Point::from(x);

            assert_eq!(p, x * G);
        }
    }

    #[test]
    fn identity() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);
            let p = Point::from(x);

            assert_eq!(p + Point::identity(), p);
        }
    }

    #[test]
    fn add() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);
            let y = Scalar::random(&mut rng);

            assert_eq!(Point::from(x) + Point::from(y), Point::from(x + y));
        }
    }

    #[test]
    fn sub() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);
            let y = Scalar::random(&mut rng);

            assert_eq!(Point::from(x) - Point::from(y), Point::from(x - y));
        }
    }

    #[test]
    fn mul() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);
            let y = Scalar::random(&mut rng);
            let px = Point::from(x);

            assert_eq!(y * px, x * y * G);
        }
    }

    #[test]
    fn add_assign() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let x = Scalar::random(&mut rng);
            let y = Scalar::random(&mut rng);
            let mut p = Point::from(x);

            p += Point::from(y);

            assert_eq!(p, Point::from(x + y));
        }
    }

    #[test]
    fn multimult() {
        let mut rng = OsRng::default();
        let n = 1024usize;

        let scalars: Vec<Scalar> = (0..n).map(|_| Scalar::random(&mut rng)).collect();
        let points: Vec<Point> = (0..n)
            .map(|_| Point::from(Scalar::random(&mut rng)))
            .collect();

        let mmp = Point::multimult(scalars.clone(), points.clone()).unwrap();

        let mut ecp = Point::identity();
        for i in 0..n {
            ecp += scalars[i] * points[i];
        }

        assert_eq!(mmp, ecp);
    }

    #[test]
    fn base58() {
        let mut rng = OsRng::default();
        let a = Point::from(Scalar::random(&mut rng));
        let s = format!("{}", &a);
        let c = Compressed::try_from(s.as_str()).unwrap();
        let b = Point::try_from(&c).unwrap();
        let t: String = c.into();
        let d = Compressed::try_from(t.as_str()).unwrap();
        let e = Point::try_from(&d).unwrap();

        assert_eq!(a, b);
        assert_eq!(a, e);
        assert_eq!(s, t);
    }
    #[test]
    #[allow(non_snake_case)]
    fn test_bip_340() {
        let mut rng = OsRng::default();

        for _ in 0..0xff {
            let a = Scalar::random(&mut rng);
            let A = Point::from(a);
            let c = A.compress();
            let x = field::Element::try_from(&c.data[1..]).unwrap();
            let B = Point::lift_x(&x).unwrap();

            if A.has_even_y() {
                assert_eq!(A, B);
            } else {
                let p = field::Element::from(field::P);

                assert_eq!(A.x(), B.x());
                assert_ne!(A.y(), B.y());
                assert_eq!(A.y(), p - B.y());
            }
        }
    }
}
