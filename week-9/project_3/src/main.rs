use std::fs::File;
use std::io::{self, Write};


struct Minister {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

impl Minister {
    // Constructor to create a new Minister
    fn new(name: &str, ministry: &str, geopolitical_zone: &str) -> Self {
        Minister {
            name: name.to_string(),
            ministry: ministry.to_string(),
            geopolitical_zone: geopolitical_zone.to_string(),
        }
    }

    // Function to save the minister's details to a file
    fn save_to_file(&self, file: &mut File) -> io::Result<()> {
        writeln!(file, "{}\t{}\t{}", self.name, self.ministry, self.geopolitical_zone)
    }
}

fn main() {
    // Create vectors for each zone's dataset
    let south_west = vec![
        Minister::new("Aigbooun Alamba Daudu", "Internal Affairs", "South West"),
        Minister::new("Adewale Jimoh Akanbi", "Power & Steel", "South West"),
    ];

    let north_east = vec![
        Minister::new("Murtala Afeez Bendu", "Justice", "North East"),
    ];

    let south_south = vec![
        Minister::new("Okorocha Calistus Ogbona", "Defense", "South South"),
    ];

    let south_east = vec![
        Minister::new("Osazuwa Faith Elieve", "Petroleum", "South East"),
    ];

    // Combine all the datasets into one vector
    let mut all_ministers = Vec::new();
    all_ministers.extend(south_west);
    all_ministers.extend(north_east);
    all_ministers.extend(south_south);
    all_ministers.extend(south_east);

    
    println!("S/N\tMINISTER NAME\t\t\tMINISTRY\t\t\t\tGEO-POLITICAL ZONE");

    
    let mut serial_number = 1;
    for minister in &all_ministers {
        println!(
            "{}\t{}\t\t{}\t\t{}",
            serial_number, minister.name, minister.ministry, minister.geopolitical_zone
        );
        serial_number += 1;
    }

    
    let file_creation = File::create("merged_ministers.txt");

    match file_creation {
        
        Result::Ok(mut file) => {
            let header = writeln!(file, "S/N\tMINISTER NAME\t\t\tMINISTRY\t\t\t\tGEO-POLITICAL ZONE");

            match header {
                
                Result::Ok(_) => {
                    let mut serial_number = 1;
                    for minister in &all_ministers {
                        let save_result = minister.save_to_file(&mut file);

                        match save_result {
                        
                            Result::Ok(_) => (),
                            
                            Result::Err(error) => eprintln!("Error saving minister data: {}", error),
                        }

                        let record_result = writeln!(file, "{}\t{}\t\t{}\t\t{}", serial_number, minister.name, minister.ministry, minister.geopolitical_zone);
                        
                        
                        match record_result {
                            Result::Ok(_) => (),
                            Result::Err(error) => eprintln!("Error writing record to file: {}", error),
                        }

                        serial_number += 1;
                    }

                    println!("\nMerged minister details saved to 'merged_ministers.txt'.");
                }
                
                Result::Err(error) => eprintln!("Error writing header to file: {}", error),
            }
        }
        
        Result::Err(error) => eprintln!("Error creating file: {}", error),
    }
}
