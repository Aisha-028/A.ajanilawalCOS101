use std::io;

fn add(a: i32, b: i32) {
	let sum = a + b;

	println!("Sum of A and B = {}", sum);
}fn main() {
	
	let mut input1 = String::new();
	println!("Enter input for parameter A:");
	io::stdin().read_line(&mut input).expect("Failed to read input");
}