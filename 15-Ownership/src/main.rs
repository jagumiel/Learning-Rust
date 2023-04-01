fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    // s is no longer valid here, as it was moved into the function
    // so this line would cause a compile-time error:
    // println!("{}", s);

    // create an integer and assign it to x
    let x = 5;

    // pass x as an argument to a function
    makes_copy(x);

    // x is still valid here, because it's a simple type that implements the Copy trait
    println!("{}", x);
}

// this function takes ownership of the String that's passed to it
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // when the function goes out of scope, some_string is dropped and its memory is freed

// this function borrows its argument instead of taking ownership
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope here, but because it's a simple type that implements the Copy trait, nothing special happens
