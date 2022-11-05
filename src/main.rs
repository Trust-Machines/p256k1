#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod bindings;

use crate::bindings::{
    secp256k1_scalar, secp256k1_scalar_set_int,
};

fn main() {
    unsafe {
        let mut scalar = secp256k1_scalar{
            d: [0, 0, 0, 0],
        };

        secp256k1_scalar_set_int(&mut scalar, 0);
    }
    println!("Hello, world!");
}
