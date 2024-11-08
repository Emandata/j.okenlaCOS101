use std::io;

fn main() {
    
    let prices = [
        ("P", 3200),
        ("F", 3000),
        ("A", 2500),
        ("E", 2000),
        ("W", 2500),
    ];

    
    let mut price_map = std::collections::HashMap::new();
    for (code, price) in prices {
        price_map.insert(code, price);
    }

    
    let mut total_cost = 0;
    loop {
        println!("Enter food code (P, F, A, E, W) or 'Q' to quit:");

        let mut food_code = String::new();
        io::stdin().read_line(&mut food_code).expect("Failed to read input");
        let food_code = food_code.trim().to_uppercase();

        if food_code == "Q" {
            break;
        }

        if let Some(&price) = price_map.get(food_code.as_str()) {
            println!("Enter quantity:");
            let mut quantity = String::new();
            io::stdin().read_line(&mut quantity).expect("Failed to read input");
            let quantity: u32 = match quantity.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid quantity. Please enter a number.");
                    continue;
                }
            };

            
            total_cost += price * quantity;
        } else {
            println!("Invalid food code. Please try again.");
        }
    }

    
    if total_cost > 10000 {
        total_cost = (total_cost as f64 * 0.95) as u32; // 5% discount
    }

    println!("Total charges: N{}", total_cost);
}
