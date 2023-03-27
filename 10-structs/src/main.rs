struct Person {
    name: String,
    surname: String,
    phone: u64,
    email: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("John"),
        surname: String::from("Smith"),
        phone: 34123123123,
        email: String::from("j.smith@rust.es"),
        age: 30,
    };

    println!("{} is {} years old", person.name, person.age);
}