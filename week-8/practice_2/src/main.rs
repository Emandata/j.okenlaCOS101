use std::io;

fn main() {
    let v = vec!['C', 'O', 'M', 'P', 'U', 'T', 'E', 'R'];

    // Prompt the user for an index value
    let input = input("Enter an index value between 0 and 7: ");
    let index: usize = input.trim().parse().expect("Please enter a valid number");

    // Check if the index is within the valid range
    if index >= v.len() {
        println!("Error: Index out of range. Please enter a value between 0 and 7.");
        return;
    }

    let ch: char = v[index];
    println!("'{}' is the character for index [{}]", ch, index);
}

// Function to handle input with a prompt
fn input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
