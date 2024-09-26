use std::io::{stdin, Read};

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();

    let a: u8 = input.next().unwrap().parse().unwrap();
    let b: u8 = input.next().unwrap().parse().unwrap();

    // if a > b {
    //     println!("{}", a);
    // } else {
    //     println!("{}", b);
    // }
    println!("{}", if a > b { a } else { b });
}
