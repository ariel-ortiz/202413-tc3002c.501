use std::num::ParseIntError;

fn main() {
    let s: String = "42".to_string();
    let res: Result<i8, ParseIntError> = s.parse::<i8>();

    match res {
        Ok(x) => println!("x = {}", x),
        Err(e) => println!("error = {}", e)
    }
}
