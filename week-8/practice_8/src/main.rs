fn main() {
    let age = vec![16,17,18,19,20,21,22,23];
    let name = vec!["Mary","Sam","Sally","Greg","Ade","Mark","June","Ife"];
    print!("\nAge allocation:\n");

    for i in 0..age.len()
    {
        print!("{} is {} years old\n",name[i],age[i]);
    }
}
