use cgmath::{prelude::*, Vector3};
use cgmath::{Deg, Rad};

fn main() {
    let v1 = Vector3::new(1.0, 1.0, 0.0);
    let v2 = Vector3::new(1.0, 0.0, 0.0);
    let a = v1.angle(v2);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("rad: {:?}", v1.angle(v2));
    println!("angle: {:?}", Deg::from(a));
    println!("distance: {:?}", v1.distance2(v2));

}
