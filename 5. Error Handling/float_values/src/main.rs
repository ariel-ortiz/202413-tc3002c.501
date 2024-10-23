use std::f32::{INFINITY, NAN};

fn main() {
    let x: f64 = 0.1;
    let y: f64 = x + x + x;
    let z: f64 = 0.3;
    println!("{y} == {z}: {}", y == z);
    println!("approx::relative_eq!({y}, {z}): {}", approx::relative_eq!(y, z));
    let a = 1.9;
    let b = 2.0;
    println!("approx::relative_eq!({}, {}): {}",
        a, b, approx::relative_eq!(a, b, epsilon = 0.2));
    println!("{}", f64::EPSILON);
    let a = 0.0;
    let b = -0.0;
    println!("{a} == {b}: {}", a == b);
    let c = 1.0;
    let d = c / a;
    let e = c / b;
    println!("d = {d}, e = {e}");
    let f = a / a;
    println!("f = {f}");
    println!("{f} == {f}: {}", f == f);
}
