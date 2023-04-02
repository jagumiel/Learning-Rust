fn main() {
    let mut x = 5;
    borrow_value(&x); // call the borrow_value function, passing a reference to x
    println!("x = {}", x); // print the value of x, which should still be 5
}

fn borrow_value(y: &i32) { // define a function that borrows an i32 value
    println!("y = {}", y); // print the value of y, which should be 5
    
    // The following line would cause a compile-time error, because y is an immutable reference:
    // *y = 10;
}
