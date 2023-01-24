# p256k1
Wrappers around secp256k1 to expose the underlying math, specifically unwrapped points and scalars with multiexponentiation.

## Dependencies
Stable rust with a working cargo.  Also need llvm to build secp256k1 and link the wrappers.

### Ubuntu
$ apt install llvm

### MacOS with brew
After installing llvm, be sure to follow the instructions on updating .zshrc so llvm can be used by the build system.

$ brew install llvm
