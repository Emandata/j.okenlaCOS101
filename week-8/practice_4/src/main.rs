fn main() {
    let mut c = ["red","gren","yellow","white"];

    print!("\nOriginal array ={:?}",c );

    let sc = &mut c[1..3];

    print!("First slice ={:?}",sc );

    sc[1] = "purple";

    println!("Change slice = {:?}",sc );
}
