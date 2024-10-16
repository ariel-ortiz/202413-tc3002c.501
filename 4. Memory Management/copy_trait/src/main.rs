#[derive(Debug, Clone, Copy)]
struct Rational {
    numerator: i32,
    denominator: i32
}

fn main() {
    let a = Rational {
        numerator: 1,
        denominator: 2
    };
    println!("a = {:?}", a);
    let mut b = a.clone();
    b.denominator = 6;
    println!("a = {:?}, b = {:?}", a, b);
    let c: Rational = a; // copy
    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);
}
