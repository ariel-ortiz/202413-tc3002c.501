#[derive(Debug)]
enum CardinalPoints {
    North, South, East, West
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Black,
    White,
    RGB(u8, u8, u8)
}

fn main() {
    let point1: CardinalPoints = CardinalPoints::North;
    let point2 = CardinalPoints::West;

    println!("point1 = {:?}, point2 = {:?}", point1, point2);

    let color1: Color = Color::Red;
    let color2: Color = Color::Black;
    let color3: Color = Color::RGB(0, 255, 255);

    println!("color1 = {:?}, color2 = {:?}, color3 = {:?}", color1, color2, color3);

    match color3 {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
        Color::RGB(r, g, b) => println!("r = {}, g = {}, b = {}", r, g, b),
        _ => println!("I don't know!")
    }
}
