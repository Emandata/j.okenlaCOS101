fn main() {
    
    let full_name = "Chibudum John Umeh";
    let department = "Computer Science";
    let uni = "Pan Atlantic University";

    let mut school = "School of Science".to_string();
    school.push_str(" and Technology");

    println!("My name is: {}", full_name );
    println!("The length my fullname is {}", full_name.len());
    println!("I am a student of {} Depratment", department );
    print!("{}",school );
    println!("\n{}",uni );
}