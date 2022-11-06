# rust-secp256k1-math
Wrappers around secp256k1 to expose the underlying math, specifically unwrapped points and scalars with multiexponentiation.

## Dependencies
Stable rust with a working cargo.  Also need llvm to build secp256k1 and the link the wrappers.

### Ubuntu
$ apt install llvm

### MacOS with brew
$ brew install llvm
