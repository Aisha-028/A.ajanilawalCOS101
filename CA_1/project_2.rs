fn main() {
	let p:f64 = 3000.0;
	let r:f64 = 10.0;
	let t:f64 = 8.0;

	//compund interest
	let a = p * (1.0 + r / 100.0).powf(t);
	println!("Amount is {}", a);
	let ci = a - p;
	println!("Compound Interest is {}", ci);

	//while true

	loop {
		let mut x = String::new();
		println!("x = {}", x);

		if x == n {
		   break;
		} 
	}
}


