fn main() {
    let a = 20;
    let b = 30;

    if (a > 10) && (b > 10) {
        print!("\ntrue");
    }
    let c = 0;
    let d = 30;

    if (c>10) || (d>10){
        print!("\ntrue");
    }
    let is_elder = false;

    if !is_elder {
        println!("\nNot elder");
    }
}
