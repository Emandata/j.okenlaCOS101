fn main() {
	let v = vec![10,20,30];
	let v2 = v;
	display(v2);
	print!("In main{:?}",v2);    
}
fn display(v:Vec<i32>){
	print!("Inside display{:?}",v);
}