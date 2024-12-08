use std::io;

fn main() {
    // Create an empty vector to store city names
    let mut city: Vec<String> = Vec::new();

    // Print initial size of the city vector
    println!("The City vector has {} elements.", city.len());

    // Ask user how many cities they want to enter
    let city_num: usize = input("How many cities do you want to enter? ")
        .trim()
        .parse()
        .expect("Invalid input. Please enter a number.");

    // Collect cities from user input
    for count in 1..=city_num {
        let new_city = input(&format!("Enter city {}: ", count));
        city.push(new_city);
    }

    // Print the collected cities
    println!("\nYour preferred cities are:");
    for (index, city_name) in city.iter().enumerate() {
        println!("{}. {}", index + 1, city_name);
    }
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
