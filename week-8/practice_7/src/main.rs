use std::io;

// Function to print the value at the specified index
fn display_value(value: Option<&char>) {
    match value {
        Some(&ch) => println!("Element of vector: {}", ch),
        None => println!("Invalid index. Please enter a number between 0 and 7."),
    }
}

fn main() {
    // Initialize a vector with characters
    let v = vec!['R', 'U', 'S', 'T', 'A', 'C', 'I', 'N'];

    // Prompt user for an index
    let index: usize = loop {
        let input = input("Enter an index value (0 - 7): ");
        match input.trim().parse::<usize>() {
            Ok(num) if num < v.len() => break num,
            _ => println!("Invalid input. Please enter a valid number between 0 and 7."),
        }
    };

    // Get the value at the specified index
    let ch = v.get(index);
    display_value(ch);
}

// Function to handle user input
fn input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

