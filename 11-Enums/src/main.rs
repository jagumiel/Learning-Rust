enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Green;

    match color {
        Color::Red => println!("The color is red!"),
        Color::Green => println!("The color is green!"),
        Color::Blue => println!("The color is blue!"),
    }
}