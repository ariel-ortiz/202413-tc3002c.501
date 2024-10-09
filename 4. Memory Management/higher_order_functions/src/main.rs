fn main() {
    let v1: Vec<i32> = vec![4, 8, 15, 16, 23, 42];
    let v2: Vec<i32> = v1.iter().map(|x| (*x) * 2).collect();
    let v3: Vec<&i32> = v1.iter().filter(|x| (*x) % 2 == 0).collect();
    let v4: Vec<f64> = v1.iter().map(|x| (*x) as f64).collect();
    let v5: Vec<&i32> = v1.iter().take(3).collect();

    println!("v1 = {v1:?}");
    println!("v2 = {v2:?}");
    println!("v3 = {v3:?}");
    println!("v4 = {v4:?}");
    println!("v5 = {v5:?}");

    let f: fn(i32) -> f64 = |x: i32| (x as f64).sqrt();
    let i: f64 = f(2);
    println!("i = {i}");
}
