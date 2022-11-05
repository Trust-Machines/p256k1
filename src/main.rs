#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod bindings;

use crate::bindings::secp256k1_scalar_one;

fn main() {
    unsafe {
        let scalar = secp256k1_scalar_one;
    }
    println!("Hello, world!");
}
