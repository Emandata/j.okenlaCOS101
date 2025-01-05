use std::fs::File;
use std::io::{self, Write};


struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

impl Student {
    // Constructor to create a new Student
    fn new(name: &str, matric_number: &str, department: &str, level: u32) -> Self {
        Student {
            name: name.to_string(),
            matric_number: matric_number.to_string(),
            department: department.to_string(),
            level,
        }
    }

    fn save_to_file(&self, file: &mut File) {
        let data = format!("{}\t{}\t{}\t{}", self.name, self.matric_number, self.department, self.level);
        
        
        file.write_all(data.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap(); 
    }
}

fn main() {
    
    let students = vec![
        Student::new("Oluchi Mordi", "ECC1011010", "Accounting", 300),
        Student::new("Allyu", "CSC10328828", "Economics", 100),
        Student::new("Shania Bolade", "EEELIN", "Computer", 200),
        Student::new("Adekunle Gold", "MEEL212001", "Electrical", 200),
        Student::new("Banca Edemoh", "MEEL212002", "Mechanical", 100),
    ];

    println!("PAU SMIS");
    println!("Student Name\tMatric. Number\tDepartment\tLevel");

    
    for student in &students {
        println!("{}\t{}\t{}\t{}", student.name, student.matric_number, student.department, student.level);
    }

    
    let mut file = File::create("students.txt").unwrap(); // This will panic on failure

    
    file.write_all(b"PAU SMIS\n").unwrap();
    file.write_all(b"Student Name\tMatric. Number\tDepartment\tLevel\n").unwrap();

    
    for student in &students {
        student.save_to_file(&mut file);
    }

    println!("\nStudent details saved to 'students.txt'.");
}
