use std::fs;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./input_1.txt")
        .expect("Should be able to read the file");
    println!("{contents}")
}
