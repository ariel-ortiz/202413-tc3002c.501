fn main() {
    let v: Vec<i8> = vec![4, 8, 15, 16, 23, 42];
    let op: Option<&i8> = v.get(3);

    // Primer opción
    // let x: i8 = *op.unwrap();
    // println!("x = {}", x);

    // Segunda opción
    // match op {
    //     Some(x) => println!("x = {}", x),
    //     None => println!("No value")
    // }

    // Tercera opción
    // if let Some(x) = op {
    //     println!("x = {}", x);
    // } else {
    //     println!("No value");
    // }

    // Cuarta opción
    if op.is_none() {
        println!("No value");
    } else {
        let x: i8 = *op.unwrap();
        println!("x = {}", x);
    }
}
