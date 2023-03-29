fn divide(x: f32, y: f32) -> Result<f32, &'static str> {
    if y == 0.0 {
        return Err("division by zero");
    }
    Ok(x / y)
}

fn main() {
    let x = 10.0;
    let y = 0.0;

    match divide(x, y) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    let num: Option<i32> = None;

    match num {
        Some(x) => println!("Number: {}", x),
        None => println!("No number found"),
    }
}
