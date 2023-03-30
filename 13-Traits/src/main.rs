trait Vehicle {
    fn drive(&self);
}

struct Car {
    model: String,
}

impl Vehicle for Car {
    fn drive(&self) {
        println!("Driving a {} car.", self.model);
    }
}

fn main() {
    let my_car = Car { model: String::from("Mazda") };
    my_car.drive();
}
