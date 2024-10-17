use std::u8;

fn main() {
    checked_demo();
    overflowing_demo();
}

fn checked_demo() {
    let x: u8 = 255;
    let y_op: Option<u8> = x.checked_add(1);
    match y_op {
        Some(y) => println!("x = {x}, y = {y}"),
        None => println!("Overflow!")
    }
}

fn overflowing_demo() {
    let x: u8 = 254;
    let (y, is_overflowed): (u8, bool) = x.overflowing_add(1);
    println!("x = {x}, y = {y}, is_overflowed = {is_overflowed}");
}
