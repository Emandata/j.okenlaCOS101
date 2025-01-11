struct Employee {
    ceo:String,
    company:String,
    age:u32
}
fn main () {
    let emp1 = Employee {
        company:String::from("Microsoft Corporation"),
        ceo:String::from("Sataya Nadella"),
        age:56
    };
    let emp2 = Employee{
        company:String::from("Google Inc."),
        ceo:String::from("Sundai Pichia"),
        age:51
    };
    display(emp1);
    display(emp2);
}
fn display (emp:Employee){
    print!("Name is:{} company is {} age is {}",emp.ceo,emp.company,emp.age);
}