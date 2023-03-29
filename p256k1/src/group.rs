use crate::_rename::{secp256k1_fe_inv, secp256k1_fe_mul, secp256k1_fe_set_int, secp256k1_fe_sqr};
use crate::bindings::{secp256k1_fe, secp256k1_ge, secp256k1_gej};

fn fe_default() -> secp256k1_fe {
    secp256k1_fe {
        n: Default::default(),
    }
}

#[inline]
pub fn secp256k1_ge_set_gej(r: &mut secp256k1_ge, a: &secp256k1_gej) {
    let mut z2 = fe_default();
    let mut z3 = fe_default();
    let mut az = fe_default();
    let mut ax = fe_default();
    let mut ay = fe_default();
    r.infinity = a.infinity;
    unsafe {
        secp256k1_fe_inv(&mut az, &a.z);
        secp256k1_fe_sqr(&mut z2, &az);
        secp256k1_fe_mul(&mut z3, &az, &z2);
        secp256k1_fe_mul(&mut ax, &a.x, &z2);
        secp256k1_fe_mul(&mut ay, &a.y, &z3);
        secp256k1_fe_set_int(&mut az, 1);
    }
    r.x = ax;
    r.y = ay;
}
