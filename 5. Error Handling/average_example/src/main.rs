fn main() {
    let v: Vec<i8> = vec![4, 8, 15, 16, 23, 42];
    let avg_op: Option<f64> = average(&v);
    if let Some(r) = avg_op {
        println!("Average of {:?} is {}", v, r);
    } else {
        println!("Can't compute average of an empty vector");
    }
    let std_dev_op: Option<f64> = standard_deviation(&v);
    match std_dev_op {
        Some(r) => println!("Standard deviation of {:?} is {}", v, r),
        None => println!("Can't compute std dev of an empty vector")
    }
}

fn average(data: &Vec<i8>) -> Option<f64> {
    let len: usize = data.len();
    if len == 0 {
        return None
    }
    let sum: i8 = data.iter().sum();
    Some(sum as f64 / len as f64)
}

fn standard_deviation(data: &Vec<i8>) -> Option<f64> {
    let avg: f64 = average(data)?;
    let sum: f64 = data.iter().map(|x| (*x as f64 - avg).powi(2)).sum();
    Some((sum / data.len() as f64).sqrt())
}
