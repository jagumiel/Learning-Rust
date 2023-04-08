use std::fs::File;
use std::io::Read;

fn main() {
    let filename = "example.txt";
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Error opening file {}: {:?}", filename, error),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => println!("File contents: {}", contents),
        Err(error) => panic!("Error reading file {}: {:?}", filename, error),
    };
}
//Try deleting "example.txt" to display the error message.