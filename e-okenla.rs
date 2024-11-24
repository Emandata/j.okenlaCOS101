use std::io;

fn main() {
    // Student Council Voter System
    println!("Student Council Voter System");

    for i in 1..=50 {
        println!("\nEnter details for candidate {}:", i);
        
        let name = input("Name");
        let email = input("Email");
        let department = input("Department");
        let state_of_origin = input("State of Origin");
        let is_class_rep = input("Are you a Class Rep? (yes/no)").to_lowercase() == "yes";
        let level: u32 = input("Enter your level").trim().parse().expect("Please enter a valid number");
        let cgpa: f32 = input("Enter your CGPA").trim().parse().expect("Please enter a valid number");

        if is_class_rep && level != 100 && cgpa > 4.0 {
            println!("You can vote");
            println!("Name: {}", name);
            println!("Email: {}", email);
            println!("Department: {}", department);
            println!("State of Origin: {}", state_of_origin);
        } else {
            println!("Sorry, you are not eligible to vote.");
        }break
    }

    // Staff Publication Incentive System
    println!("\nStaff Publication Incentive System");

    for i in 1..=100 {
        println!("\nEnter details for staff member {}:", i);
        
        let name = input("Name");
        let num_papers: u32 = input("Number of papers published").trim().parse().expect("Please enter a valid number");
        
        let incentive = match num_papers {
            0..=2 => 100_000,
            3..=5 => 500_000,
            6..=9 => 800_000,
            10..=u32::MAX => 1_000_000,
        };

        println!("Name: {}", name);
        println!("Incentive: ₦{}", incentive);
    }
}

fn input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}: ", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()

}
