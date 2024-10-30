<<<<<<< HEAD
fn  main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	let a = p * ( 1.0 + (r / 100.0)).powf(t);
	println!("Amount is {}", a );
	let ci = a - p;
	println!("Compound Interest is {}", ci );
	let d = p - ci;
	println!("depriciation is {}", d );

=======
fn  main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	let a = p * ( 1.0 + (r / 100.0)).powf(t);
	println!("Amount is {}", a );
	let ci = a - p;
	println!("Compound Interest is {}", ci );
	let d = p - ci;
	println!("depriciation is {}", d );

>>>>>>> a9f01e0ac16c10b0551cb03ffbc453521d26ab87
}