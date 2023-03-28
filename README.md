# p256k1

[![ci](https://github.com/Trust-Machines/p256k1/actions/workflows/ci.yml/badge.svg)](https://github.com/Trust-Machines/p256k1/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/p256k1.svg)](https://crates.io/crates/p256k1)

**Rust wrappers around libsecp256k1 to expose unwrapped points and scalars with multi-exponentiation**

`p256k1` is a library providing group operations on secp256k1, a prime order Weierstrass curve.

`p256k1` provides access to curve points in Jacobian coordinates, which allows for very fast operations.  Standard math operators are implemented to allow writing code which closely resembles the underlying math.  The `BitXor` operator is used for scalar exponentiation, using a fast square and multiply algorithm; this provides a very natural and intuitive API.  Fast multi-exponentiation is provided using the standard Pippenger algorithm.

See [p256k1/](p256k1) for more details.
