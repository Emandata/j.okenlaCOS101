fn main() {
    let mut mh = ("Everest", 8848, "Fishtail", 6993);
    print!("Original tuple = {:?}",mh );

    mh.2 = "Lhoste";
    mh.3 = 8516;

    print!("Changed tuple ={:?}",mh );
    
}
