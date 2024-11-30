use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
fn trapezium() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = input("input the value of height: ").trim().parse().expect("Failed to read value");
     let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f32 = input("input the value of base 1: ").trim().parse().expect("Failed to read value");
     let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f32 = input("input thevalue of base 2: ").trim().parse().expect("Failed to read value");
    let area:f64 = (a/2.0*(b+c)) as f64;
    print!("area of the trapezium is: {}", area);
}    
fn rhombus () {
     let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = input("input the first diagonal: ").trim().parse().expect("Failed to read value");
     let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f32 = input("input the second diagonal: ").trim().parse().expect("Failed to read value");
    let area = (0.5 * a * b) as f64;
    println!("area of rhombus is: {}",area );
}
fn parallelogram () {
     let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f32 = input("input the base: ").trim().parse().expect("Failed to parse");
     let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f32 = input("input the altitude: ").trim().parse().expect("Failed to parse");
    let area = (a * b) as f64;
    println!("area of parallelogram is: {}", area);
}
fn cube () {
     let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = input("input the length of the side: ").trim().parse().expect("Failed to read value");
    let area = (6.0 * a * a) as f64 ;
    println!("area of cube is: {}",area );    
}    
fn cylinder () {
     let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = input("input the value of radius: ").trim().parse().expect("Failed to read value");
     let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f32 = input("input the value of height: ").trim().parse().expect("Failed to reasd value");
    let volume = (3.14 * a * a * b) as f64;
    println!("area of cylinder is: {}",volume);
}        
fn main() {
    loop {
        println!("What area of shape would you like to solve?");
        println!("");
        println!("Trapezium       1");
        println!("rhombus         2");
        println!("cube            3");
        println!("cylinder        4");
        println!("parallelogram   5");
        println!("press Q to quit");


         let mut num_code = String::new();
        io::stdin()
            .read_line(&mut num_code)
            .expect("Failed to read input");
        let num_code = num_code.trim().to_uppercase();

        if num_code == "Q" {
            break;
        }

        match num_code.as_str() {
            "1" => trapezium(),
            "2" => rhombus(),
            "3" => cube(),
            "4" => cylinder(),
            "5" => parallelogram(),
            _ => {
                println!("Invalid number code. Please try again.");
                continue;
            }
        }

    } 
}









