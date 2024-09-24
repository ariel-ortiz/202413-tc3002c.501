fn main() {
    hola();
    let n: u8 = 34;
    let x: u128 = fact(n);
    println!("x = {}", x);

    let s1: i32 = sign(5);
    let s2: i32 = sign(-3);
    let s3: i32 = sign(0);
    println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);

    let n = -100;
    match sign(n) {
        -1 => println!("{} es negativo", n),
        1 => println!("{} es positivo", n),
        _ => println!("{} es cero", n)
    }
}

fn hola() {
    println!("¡Hola, mundo!");
}

fn fact(n: u8) -> u128 {
    let mut result: u128 = 1;

    // for i in 2..=n {
    //     result *= i as u128;
    // }

    let mut i: u8 = 2;

    // while i <= n {
    //     result *= i as u128;
    //     i += 1;
    // }

    loop {
        if i > n {
            break;
        }
        result *= i as u128;
        i += 1;
    }

    result
}

fn sign(n: i32) -> i32 {
    if n < 0 {
        -1
    } else if n > 0 {
        1
    } else {
        0
    }
}
