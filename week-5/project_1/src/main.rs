use std::io;

fn main() {
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    let mut total_cost = 0;

    loop {
        println!("Enter food code (P, F, A, E, W) or 'Q' to quit:");

        let mut food_code = String::new();
        io::stdin()
            .read_line(&mut food_code)
            .expect("Failed to read input");
        let food_code = food_code.trim().to_uppercase();

        if food_code == "Q" {
            break;
        }

        
        let price = match food_code.as_str() {
            "P" => 3200,
            "F" => 3000,
            "A" => 2500,
            "E" => 2000,
            "W" => 2500,
            _ => {
                println!("Invalid food code. Please try again.");
                continue;
            }
        };

        let quantity: u32;
        loop {
            println!("Enter the portion of food wanted:");

            let mut quantity_input = String::new();
            io::stdin()
                .read_line(&mut quantity_input)
                .expect("Failed to read input");

            let parsed_quantity = quantity_input.trim().parse::<u32>();
            if parsed_quantity.is_ok() {
                quantity = parsed_quantity.unwrap();
                break;
            } else {
                println!("Invalid quantity. Please enter a valid number.");
            }
        }

        total_cost += price * quantity;
    }    


        if total_cost > 10_000 {
            total_cost = (total_cost as f64 * 0.95) as u32;
        }


       println!("Total charges: N{}", total_cost);
}