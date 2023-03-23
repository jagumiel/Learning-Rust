fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for number in numbers.iter() {
        println!("{}", number);
    }
    println!("The second value of the array is: {}", numbers[1]);
}