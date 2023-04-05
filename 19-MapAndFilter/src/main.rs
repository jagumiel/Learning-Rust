fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=n/2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Use map to create a new vector with each element squared
    let squared_numbers: Vec<i32> = numbers.iter().map(|x| x * x).collect();

    // Use filter to create a new vector with only even numbers
    let even_numbers = numbers.iter().filter(|&&x| x % 2 == 0).collect::<Vec<_>>();

    // Use filter to create a new vector with prime even numbers
    let primes = numbers.iter().filter(|&&x| is_prime(x) == true).collect::<Vec<_>>();

    println!("Original vector:  {:?}", numbers);
    println!("Squared vector:   {:?}", squared_numbers);
    println!("Even vector:      {:?}", even_numbers);
    println!("Primes vector:    {:?}", primes);
}
