fn main() {
    // Initialize a tuple
    let b: (i32, bool, f64) = (30, true, 4.9);
    
    // Call the print function with the tuple
    print(b);
}

// Function to print tuple elements
fn print(x: (i32, bool, f64)) {
    println!("Inside print method");

    // Destructure the tuple into distinct variables
    let (age, is_male, cgpa) = x;

    // Print the destructured values
    println!("Age is {}, isMale? {}, CGPA is {}", age, is_male, cgpa);
}
