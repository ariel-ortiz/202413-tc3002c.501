fn main() {
    let mut a: Vec<String> = vec![
        "enero".to_string(),
        "febrero".to_string(),
        "marzo".to_string(),
        "abril".to_string()
    ];
    let b: Vec<String> = a.clone();
    let c: Vec<String> = a.clone();
    let d: Vec<String> = a.clone();
    ownership_and_borrow(&mut a);
    for_ver1(b);
    for_ver2(c);
    for_ver3(d);
}

fn ownership_and_borrow(a: &mut Vec<String>) {
    let s: &String = &a[0];
    println!("{s}");
    println!("{a:?}");

    let s: String = a.pop().unwrap();
    println!("{s}");
    println!("{a:?}");

    let s: String = a.remove(0);
    println!("{s}");
    println!("{a:?}");
}

fn for_ver1(v: Vec<String>) {
    // For con "ownership"
    for elem in v.into_iter() { // for elem in v { ... }
        println!("{elem}");
    }
    // v ya no es dueña del vector ni de sus elementos
}

fn for_ver2(v: Vec<String>) {
    // For con "borrowing" de solo lectura
    for elem in v.iter() {  // for elem in &v { ... }
        println!("{}", *elem);
    }
    println!("{v:?}");
}

fn for_ver3(v: Vec<String>) {
    let mut n = v.clone();
    // For con "borrowing" de lectura/escritura
    for elem in n.iter_mut() { // for elem in &mut n { ... }
        elem.push('*');
        println!("{elem}");
    }
    println!("{n:?}");
}
