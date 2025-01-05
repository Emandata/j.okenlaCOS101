use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() {
    let mut file = File::create("welcome_message.txt").expect("Failed to create file");
    file.write_all("Welcome to Rust Programming!\n").expect("Failed to write to file");
    println!("File created successfully!");


    let mut file = std::fs::File::open("welcome_message.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}",contents);
}