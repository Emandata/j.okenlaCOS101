fn main() {
    let x = vec![100,200,300];
    borrow_vector(&x);
    print!("Printing the value from main() x [0] = {}",x[0] );
    print!("******************************");
}

fn borrow_vector(z:&Vec<i32>){

    print!("**********************");
    print!("Inside print_vector function {:?} \n",z );
    print!("------------------------------");
}