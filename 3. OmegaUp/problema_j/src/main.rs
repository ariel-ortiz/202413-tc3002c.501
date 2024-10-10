use std::io::stdin;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_ascii_whitespace();
    let n: u16 = input.next().unwrap().parse().unwrap();
    let mut v: Vec<(u8, u32)> = Vec::new();
    v.reserve(1000);
    for _ in 0..n {
        let x: u32 = input.next().unwrap().parse().unwrap();
        v.push((cuenta_bits_uno(x), x));
    }
    v.sort();
    for x in v {
        print!("{} ", x.1);
    }
}

fn cuenta_bits_uno(mut x: u32) -> u8 {
    let mut bits: u8 = 0;
    while x != 0 {
        bits += (x & 1) as u8;
        x >>= 1;
    }
    bits
}
