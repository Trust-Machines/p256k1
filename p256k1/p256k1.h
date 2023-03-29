#pragma once

#include "./_secp256k1/include/secp256k1.h"
#include "./_secp256k1/src/group.h"

#ifdef __cplusplus
extern "C" {
#endif

SECP256K1_API void secp256k1_ge_set_gej_const(secp256k1_ge *r, const secp256k1_gej *a);

#ifdef __cplusplus
}
#endif
