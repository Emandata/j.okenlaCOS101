fn main() {
    let name = "Aisha Lawal";
    let uni:&str = "Pan Atlasntic University";
    let addr:&str = "KM 52 Lekki-Epe Expressway, Ibeju lekki, Lagos";
    println!("Name: {}",name);
    println!("University: {}, \nAdress: {}",uni,addr );

    

    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("department: {}, \nSchool: {}",department,school); 
}
