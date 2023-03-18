fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let a = 10;
    let b = 20;

    let result = add(a, b);

    println!("The sum of {} and {} is {}", a, b, result);
}
