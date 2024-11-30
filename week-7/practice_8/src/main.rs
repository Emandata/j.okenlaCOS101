fn main() {
    let city_arr: [&str; 5] = ["Abuja", "Portharcourt", "Maiduguri", "Kano", "Lagos"];
    println!("Array is: {:?}", city_arr);
    println!("Array size is: {}", city_arr.len());

    for index in 0..5 {
        println!("City index {} is located in: {}", index, city_arr[index]);
    }
}
fn main() {
    let arr: [i32; 4] = [10, 20, 30, 40];
    println!("Array is: {:?}", arr);
    println!("Array size is: {}", arr.len());

    for val in arr.iter() {
        println!("Value is: {}", val);
    }
}

