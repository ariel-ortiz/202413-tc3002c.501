fn main() {
    let s = "19999".to_string();
    let v: Vec<u8> = s.chars().rev().map(|x| x as u8 - '0' as u8).collect();
    println!("{v:?}");
}
