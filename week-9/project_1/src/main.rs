use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Category {
    name: String,
    drinks: Vec<String>,
}

impl Category {
    fn new(name: &str, drinks: Vec<&str>) -> Self {
        Category {
            name: name.to_string(),
            drinks: drinks.into_iter().map(|s| s.to_string()).collect(),
        }
    }dgh cx

    fn save_to_file(&self, file_name: &str) {
        let mut file = File::create(file_name).unwrap(); // This will panic if an error occurs
        writeln!(file, "Category: {}", self.name).unwrap(); // This will panic if an error occurs
        writeln!(file, "Drinks:").unwrap(); // This will panic if an error occurs
        for drink in &self.drinks {
            writeln!(file, "- {}", drink).unwrap(); // This will panic if an error occurs
        }
    }
}

fn main() {

    let stout = Category::new("Stout", vec!["Legend", "Turbo King", "Williams"]);
    let non_alcoholic = Category::new("Non-Alcoholic", vec!["Maltina", "Fayrouz", "Malta Gold"]);
    let lager = Category::new("Lager", vec!["33 Export", "Goldberg", "Amstel Malta", "Gulder", "Heineken", "Star", "Desperados"]);

    
    stout.save_to_file("stout.txt");
    non_alcoholic.save_to_file("non_alcoholic.txt");
    lager.save_to_file("lager.txt");

    println!("Categories saved successfully.");
}

