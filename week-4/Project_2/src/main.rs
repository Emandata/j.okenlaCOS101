use std::io;
fn main() {
    let mut input = String::new();

    println!("\nEnter your age: ");
    io::stdin().read_line(&mut input).expect("Nota valid string");
    let age:f32 = input.trim().parse().expect("Not a valid number");
 
    if age >= 40.0
    {
        println!("your incentive is N1,560,00");

    }
    else if age >= 30.0 &&  age <= 41.0
    {
        println!("your incentive is N1,480,000");
    }
    else if age >= 21.0 && age <= 28.0
    {
        println!("your incentive is N1,300,000");
    }
    else {
        println!("your incentive should be N100,000");
    }

    
}
