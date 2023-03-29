#include "./p256k1.h"
#include "./_secp256k1/src/group_impl.h"
#include "./_secp256k1/src/field_impl.h"
#include "./_secp256k1/src/int128_impl.h"

void secp256k1_ge_set_gej_const(secp256k1_ge *r, const secp256k1_gej *a) {
    secp256k1_fe z2, z3, az, ax, ay;
    r->infinity = a->infinity;
    secp256k1_fe_inv(&az, &a->z);
    secp256k1_fe_sqr(&z2, &az);
    secp256k1_fe_mul(&z3, &az, &z2);
    secp256k1_fe_mul(&ax, &a->x, &z2);
    secp256k1_fe_mul(&ay, &a->y, &z3);
    secp256k1_fe_set_int(&az, 1);
    r->x = ax;
    r->y = ay;
}
