# p256k1

[![ci](https://github.com/Trust-Machines/p256k1/actions/workflows/ci.yml/badge.svg)](https://github.com/Trust-Machines/p256k1/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/p256k1.svg)](https://crates.io/crates/p256k1)

Wrappers around secp256k1 to expose the underlying math, specifically unwrapped points and scalars with multiexponentiation.

## Dependencies
Stable rust with a working cargo.  Also need llvm to build secp256k1 and link the wrappers.

### Ubuntu

```
$ apt install llvm
```

### MacOS with brew
After installing llvm, be sure to follow the instructions on updating .zshrc so llvm can be used by the build system.

```
$ brew install llvm
```