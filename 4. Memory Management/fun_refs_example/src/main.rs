fn main() {
    let v: Vec<u8> = vec![4, 8, 15, 16, 23, 42];
    let u: Vec<u8> = v; // move
    let w: Vec<u8>;
    w = u; // move
    println!("{}", w[0]);

    let x = fun1(w); // move
    println!("{}", x[0]);

    fun2(&x);
    println!("{}", x[0]);

    let mut y: Vec<u8> = x.clone();
    fun3(&mut y);
    println!("{:?}", y);

    let mut z: i32 = 10;
    println!("{}", z);
    fun4(&mut z);
    println!("{}", z);
}

fn fun1(a: Vec<u8>) -> Vec<u8> {
    println!("{}", a[0]);
    a // move
}

fn fun2(b: &Vec<u8>) {
    println!("{}", (*b)[0]);
}

fn fun3(c: &mut Vec<u8>) {
    (*c)[0] = 108;
}

fn fun4(i: &mut i32) {
    (*i) += 1;
}
