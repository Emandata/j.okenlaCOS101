
fn main() {
    // An array of numbers
    let numbers = [1, 2, 3, 4, 5];
    println!("Original array: {:?}", numbers);

    let slice1 = &numbers[1..3];
    println!("2nd and 3rd elements sliced: {:?}", slice1);

    // Create a slice from the start to index 3 (exclusive)
    let slice2 = &numbers[..3];
    println!("Start to index 3 sliced: {:?}", slice2);
