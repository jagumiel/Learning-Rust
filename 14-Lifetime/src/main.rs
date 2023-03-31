fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from("world!");
    let result = longest(&s1, &s2);
    if result == "" {
        println!("The strings have the same length.");
    } else {
        println!("The longest string is {}", result);
    }
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else if s2.len() > s1.len() {
        s2
    } else {
        ""
    }
}