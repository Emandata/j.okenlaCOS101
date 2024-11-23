fn main() {
    let name1 = "Ayomide Adesokan";
    print!("My name is {}",name1 );

    let name2 = name1.replace("Ayomide" , "Adebare");
    print!("\nYou can also call me {}",name2 );
    let faculty = "Faculty of Science and Technoology";

    let school = faculty.replace("Faculty","School");
    println!("\nI am a student of the {}", school );
    
}
