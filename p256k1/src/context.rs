use crate::bindings::{secp256k1_context, secp256k1_scratch_space, SECP256K1_CONTEXT_SIGN};

use super::_rename::{secp256k1_context_create, secp256k1_context_destroy, secp256k1_scratch_space_create, secp256k1_scratch_space_destroy};

/**
Context is a wrapper around libsecp256k1's internal secp256k1_context struct.
*/
pub struct Context {
    /// The wrapped libsecp256k1 context
    pub context: *mut secp256k1_context,
}

impl Default for Context {
    /// Construct a default wrapped secp256k1 context
    fn default() -> Self {
        Self {
            context: unsafe { secp256k1_context_create(SECP256K1_CONTEXT_SIGN) },
        }
    }
}

impl Drop for Context {
    /// Destroy underlying secp256k1_context ptr
    fn drop(&mut self) {
        unsafe { secp256k1_context_destroy(self.context) }
    }
}

const SCRATCH_SPACE_DEFAULT_SIZE: usize = 1048576;

/**
Scratch is a wrapper around libsecp256k1's internal secp256k1_scratch struct.
*/
pub struct Scratch {
    /// The wrapped libsecp256k1 scratch
    pub scratch: *mut secp256k1_scratch_space,
    context: Context,
}

impl Scratch {
    /// Construct a wrapped secp256k1 scratch space with the passed size
    pub fn new(size: usize) -> Self {
        let context: Context = Default::default();
        Self {
            scratch: unsafe { secp256k1_scratch_space_create(context.context, size) },
            context,
        }
    }
}

impl Default for Scratch {
    /// Construct a default wrapped secp256k1 scratch space
    fn default() -> Self {
        let context: Context = Default::default();
        Self {
            scratch: unsafe { secp256k1_scratch_space_create(context.context, SCRATCH_SPACE_DEFAULT_SIZE) },
            context,
        }
    }
}

impl Drop for Scratch {
    /// Destroy underlying secp256k1_scratch_space ptr
    fn drop(&mut self) {
        unsafe { secp256k1_scratch_space_destroy(self.context.context, self.scratch) }
    }
}
