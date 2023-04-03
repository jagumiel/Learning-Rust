fn main() {
    let mut s1 = String::from("hello");     // s1 owns the heap-allocated string "hello"
    let s2 = take_ownership(s1);            // s2 takes ownership of s1's string
    println!("s2: {}", s2);                 // s2's string is printed
    let s3 = borrow_string(&s2);            // s3 borrows s2's string
    println!("s3: {}", s3);                 // s3's borrowed string is printed
}                                           // s2 and s3 go out of scope, but s2 owns the string and it's not dropped until it goes out of scope

fn take_ownership(s: String) -> String {    // s takes ownership of a string
    println!("s1: {}", s);                  // the string is printed
    s                                       // ownership of the string is returned. If this is commented, error: "consider returning the local binding `s`"
}

fn borrow_string(s: &String) -> &str {      // s borrows a string
    &s[0..3]                                // a slice of the borrowed string is returned
}
