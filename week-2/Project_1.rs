<<<<<<< HEAD
fn  main() {
	let p:f64 = 520000000.0;
	let r:f64 =10.0;
	let t:f64 =5.0;

	// compound interest
	let a = p * ( 1.0 + (r / 100.0)).powf(t);
	println!("Amount is {}", a );
	let ci = a-p;
	println!("Compound interest is {}", ci );

=======
fn  main() {
	let p:f64 = 520000000.0;
	let r:f64 =10.0;
	let t:f64 =5.0;

	// compound interest
	let a = p * ( 1.0 + (r / 100.0)).powf(t);
	println!("Amount is {}", a );
	let ci = a-p;
	println!("Compound interest is {}", ci );

>>>>>>> a9f01e0ac16c10b0551cb03ffbc453521d26ab87
}