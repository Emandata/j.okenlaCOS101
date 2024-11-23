use std::collections::HashMap;
use std::io;

fn main() {
    
    let price_map: HashMap<&str, u32> = [
        ("P", 3200), 
        ("F", 3000), 
        ("A", 2500), 
        ("E", 2000), 
        ("W", 2500), 
    ]
    .iter()
    .cloned()
    .collect();

    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    let mut total_cost = 0;

    loop {
        
        println!("\nEnter food code (P, F, A, E, W) or 'Q' to quit:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let food_code = input.trim().to_uppercase();

        if food_code == "Q" {
            break;
        }

        
        if let Some(&price) = price_map.get(food_code.as_str()) {
            println!("Enter quantity:");
            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).expect("Failed to read input");

            if let Ok(qty) = quantity.trim().parse::<u32>() {
                total_cost += price * qty;
            } else {
                println!("Invalid quantity. Please enter a number.");
            }
        } else {
            println!("Invalid food code. Please try again.");
        }
    }

    
    if total_cost > 10_000 {
        total_cost = (total_cost as f64 * 0.95) as u32;
    }

    println!("\nTotal charges: N{}", total_cost);
}



