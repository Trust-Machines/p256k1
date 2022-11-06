#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
mod bindings;
mod scalar;
mod point;

use crate::{
    scalar::Scalar,
    point::Point,
};

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(unused_variables)]
#[allow(non_upper_case_globals)]
fn main() {
    let G: Point = Point::from(Scalar::from(1));

    assert_eq!(Scalar::from(32) + Scalar::from(10), Scalar::from(42));
    assert_eq!(Scalar::from(32) * Scalar::from(10), Scalar::from(320));
    assert_eq!(Scalar::from(52) - Scalar::from(10), Scalar::from(42));

    println!("Scalar(42) bytes {}", Scalar::from(42));

    println!("G {:?}", G);
    
    assert_eq!(&G + &Point::new(), G);
}
