#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(deref_nullptr)]
#[allow(improper_ctypes)]
#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
mod _rename;

/// secp256k1 context operations
pub mod context;

/// ECDSA operations
pub mod ecdsa;

/// Errors
pub mod errors;

/// Point operations on the secp256k1 curve
pub mod point;

/// Scalar arithmetic modulo the group order
pub mod scalar;

/// Field elements arithmetic
pub mod field;

/// Traits
pub mod traits;

mod group;
