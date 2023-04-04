struct Person {
    name: String,
    surname: String,
    phone: u64,
    email: String,
    age: u32,
}

fn main() {
    let people = vec![
        Person { name: String::from("John"), surname: String::from("Smith"), phone: 34123123123, email: String::from("j.smith@rust.es"), age: 30},
        Person { name: String::from("Alice"), surname: String::from("Alison"), phone: 34123789456, email: String::from("a.alison@trusty.com") ,age: 25 },
        Person { name: String::from("Bob"), surname: String::from("Callahan"), phone: 34654321987, email: String::from("b.callahan@noob.au"), age: 30 },
        Person { name: String::from("Trudy"), surname: String::from("Evil"), phone: 34666999333, email: String::from("t.evil@danger.me"), age: 18 },
    ];



    for person in people.iter() {
        println!("Name: {}, Age: {}", person.name, person.age);
    }
}
