use std::io;

fn main() {
    let mut developers = vec![];

    println!("Enter details of 4 developers:");

    
    for i in 1..=4 {
        println!("Developer {}:", i);

    
        println!("Enter name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        let name = name.trim().to_string();

        
        println!("Enter years of experience:");
        let mut experience = String::new();
        io::stdin().read_line(&mut experience).unwrap();
        let experience: u32 = experience.trim().parse().unwrap_or(0);

        
        developers.push((name, experience));
    }

    
    let mut most_experienced = &developers[0];
    for developer in &developers {
        if developer.1 > most_experienced.1 {
            most_experienced = developer;
        }
    }


    println!(
        "The most experienced developer is {} with {} years of experience.",
        most_experienced.0, most_experienced.1
    );
}
