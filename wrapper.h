#define ECMULT_GEN_PREC_BITS 4
#define ECMULT_WINDOW_SIZE 15

#include "secp256k1/include/secp256k1.h"
#include "secp256k1/src/scalar.h"
#include "secp256k1/src/group.h"
#include "secp256k1/src/field_impl.h"
#include "secp256k1/src/scalar_impl.h"
#include "secp256k1/src/group_impl.h"
#include "secp256k1/src/ecmult_impl.h"
#include "secp256k1/src/ecmult_const_impl.h"
#include "secp256k1/src/ecmult_gen_impl.h"
#include "secp256k1/src/eckey_impl.h"
#include "secp256k1/src/scratch_impl.h"
