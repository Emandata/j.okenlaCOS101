fn main() {
    let v = vec![20,40,60,80];

    let v2 = v;
    let v2_rturn = display(v2);
    print!("In main {:?}",v );
}
fn display(v:vec<i32>)->Vec<i32> {
    print!("inside display{:?}",v );
    return v:;
}
