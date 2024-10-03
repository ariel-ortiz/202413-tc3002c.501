fn main() {

    // Ejemplos de "Ownership"
    let v: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    let u: Vec<i32> = v; // move
    let w: Vec<i32> = u.clone();
    println!("{:?}", u);
    println!("{:?}", w);

    // Ejemplos de "Borrowing"
    let s: String = "¡Hola, Mundo!".to_string();
    let r: &String = &s;  // pedir prestado
    let p: &String = &s;  // pedir prestado
    println!("La dueña: {}", s);
    println!("Préstamo: {}", r);
    println!("Préstamo: {}", p);

    let mut a: String = "Hola".to_string();
    a.push_str(" a todos");
    println!("{}", a);
    let b: &mut String = &mut a;
    // let c = &mut a;
    b.push_str(" y esto se acabó");
    println!("{}", a);
}

// fn referencia_mala() -> &String {
//     let s: String = "Hola".to_string();
//     let r: &String = &s;
//     r
// }
