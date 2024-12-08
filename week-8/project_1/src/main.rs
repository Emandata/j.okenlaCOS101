use std::io;

fn main() {
    let position = vec![
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
        "EL2 10-13",
        "SES",
    ];

    loop {
        println!("What is your profession?");
        println!("1. Officer Admin");
        println!("2. Academic");
        println!("3. Lawyer");
        println!("4. Teacher");
        println!("Q. Quit");

        let choice = input("Enter your choice: ").to_uppercase();

        if choice == "Q" {
            println!("Goodbye!");
            break;
        }

        match choice.as_str() {
            "1" => officer_admin(&position),
            "2" => academic(&position),
            "3" => lawyer(&position),
            "4" => teacher(&position),
            _ => println!("Invalid choice! Please try again."),
        }
    }
}

fn academic(position: &Vec<&str>) {
    let status = input("What is your status in the Academic section: ").to_lowercase();
    if status == "dean" {
        println!("Your position is: {}", position[5]);
    } else {
        let years: f32 = input("How many years have you been working: ")
            .trim()
            .parse()
            .expect("Invalid input, please enter a number.");

        print_position_by_years(years, position);
    }
}

fn lawyer(position: &Vec<&str>) {
    let status = input("What is your status in the Lawyer section: ").to_lowercase();
    if status == "partner" {
        println!("Your position is: {}", position[5]);
    } else {
        let years: f32 = input("How many years have you been working: ")
            .trim()
            .parse()
            .expect("Invalid input, please enter a number.");

        print_position_by_years(years, position);
    }
}

fn officer_admin(position: &Vec<&str>) {
    let status = input("What is your status in the Office Admin section: ").to_lowercase();
    if status == "ceo" {
        println!("Your position is: {}", position[5]);
    } else {
        let years: f32 = input("How many years have you been working: ")
            .trim()
            .parse()
            .expect("Invalid input, please enter a number.");

        print_position_by_years(years, position);
    }
}

fn teacher(position: &Vec<&str>) {
    let status = input("What is your status in the Teacher section: ").to_lowercase();
    if status == "principal" {
        println!("Your position is: {}", position[5]);
    } else {
        let years: f32 = input("How many years have you been working: ")
            .trim()
            .parse()
            .expect("Invalid input, please enter a number.");

        print_position_by_years(years, position);
    }
}

fn print_position_by_years(years: f32, position: &Vec<&str>) {
    if years >= 1.0 && years <= 2.0 {
        println!("Your position is: {}", position[0]);
    } else if years >= 3.0 && years <= 5.0 {
        println!("Your position is: {}", position[1]);
    } else if years >= 6.0 && years <= 8.0 {
        println!("Your position is: {}", position[2]);
    } else if years >= 9.0 && years <= 10.0 {
        println!("Your position is: {}", position[3]);
    } else if years >= 11.0 && years <= 13.0 {
        println!("Your position is: {}", position[4]);
    } else {
        println!("Sorry, the number of years provided cannot be processed.");
    }
}

fn input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
