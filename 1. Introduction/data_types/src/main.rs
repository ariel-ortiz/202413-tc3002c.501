fn main() {
    let mut n: i8 = 42;
    println!("n = {}", n);
    n *= 2;
    println!("n = {}", n);

    let m: i32 = 10;
    let z: i32 = n as i32 + m;
    println!("z = {}", z);
}
