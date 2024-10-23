
fn main() {
    checked_demo();
    overflowing_demo();
    saturating_demo();
    wrapping_demo();
    unwrap_demo();
    let n = 35;
    println!("{n}! = {:?}", fact(n));
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
    let x: u8 = 255;
    let (y, is_overflowed): (u8, bool) = x.overflowing_add(1);
    println!("x = {x}, y = {y}, is_overflowed = {is_overflowed}");
}

fn saturating_demo() {
    let x: u8 = 255;
    let y: u8 = x.saturating_add(1);
    println!("x = {x}, y = {y}");
}

fn wrapping_demo() {
    let x: u8 = 255;
    let y: u8 = x.wrapping_add(1);
    println!("x = {x}, y = {y}");
}

fn unwrap_demo() {
    let x: u8 = 255;
    let y: u8 = x.checked_add(1).unwrap_or(200);
    println!("x = {x}, y = {y}");
}

fn fact(n: u8) -> Option<u128> {
    let mut r: u128 = 1;
    for i in 2..=n {
        match r.checked_mul(i as u128) {
            Some(x) => r = x,
            None => return None
        }
    }
    Some(r)
}
