use crate::bindings::{
    secp256k1_context, secp256k1_context_create, secp256k1_context_destroy, SECP256K1_CONTEXT_SIGN,
};

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
