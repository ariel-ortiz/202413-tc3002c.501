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

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64
}

fn main() {
    let s = Square::new(3.0);
    figure_report(&s);

    let c = Circle::new(10.0);
    figure_report(&c);

    let t = Triangle::new(4.5, 2.0);
    figure_report(&t);

    let x: f64 = 42.0;
    figure_report(&x);
}

fn figure_report<T: Figure>(figure: &T) {
    println!("FIGURE: {}", figure.description());
    println!("---------------------------------------");
    println!("Area = {:.2} m^2", figure.area());
    println!("Perímtero = {:.2} m", figure.perimeter());
    println!();
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

impl Circle {
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

    fn description(&self) -> String {
        format!("Circle (radius = {:.2} m)", self.radius)
    }
}

impl Triangle {
    fn new(base: f64, height: f64) -> Self {
        Self { base, height }
    }
}

impl Figure for Triangle {

    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }

    fn perimeter(&self) -> f64 {
        self.base * 3.0
    }

    fn description(&self) -> String {
        format!("Triangle (base = {} m, height = {} m)", self.base, self.height)
    }
}

impl Figure for f64 {

    fn area(&self) -> f64 {
        f64::NAN
    }

    fn perimeter(&self) -> f64 {
        f64::NAN
    }

    fn description(&self) -> String {
        format!("f64 ({:.2})", self)
    }
}
