use std::f64::consts::PI;

trait Figure {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn description(&self) -> String {
        "unknown".to_string()
    }
}

#[derive(Debug)]
struct Square {
    side: f64
}

#[derive(Debug)]
struct Circle {
    radius: f64
}

fn main() {
    let s = Square::new(3.0);
    println!("{:?}", s);
    println!("Area = {}", s.area());
    println!("Perímetero = {}", s.perimeter());
    println!("Descripción = {}", s.description());

    let c = Circle::new(10.0);
    println!("{:?}", c);
    println!("Area = {}", c.area());
    println!("Perímetero = {}", c.perimeter());
    println!("Descripción = {}", c.description());
}

impl Square {
    fn new(side: f64) -> Self {
        Self { side }
    }
}

impl Figure for Square {

    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }

    fn description(&self) -> String {
        format!("Square (side = {:.2} m)", self.side)
    }
}

impl  Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Figure for Circle {

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        PI * 2.0 * self.radius
    }
}
