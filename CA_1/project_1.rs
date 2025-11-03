use std::io;

fn main() {
	println!("\nStudent Grade Evaluator");

	//input name
	println!("\nPlease Enter your name.");
	let mut name = String::new();
	    let _ = io::stdin()
	    .read_line(&mut name);
	println!("Your name is {}", name);

	//Rust program to calculate average
	let mut _test1 = String::new();
	let mut _test2 = String::new();
	let mut _test3 = String::new();

	println!("First test score: ");
	let _ = io::stdin().read_line(&mut _test1);
	let _test1:f64 = 90.0;

	println!("Second test score: ");
	let _ = io::stdin().read_line(&mut _test2);
	let _test2:f64 = 87.0;

	println!("Third test score: ");
	let _ = io::stdin().read_line(&mut _test3);
	let _test3:f64 = 96.0;

	let a:f64 = (90.0 + 87.0 + 96.0) / 3.0;

	println!("Average of test scores: {}", a);

	if a >= 70.0
	{
		println!("Grade = A");
	}
	else if a >= 60.0 && a <= 69.0
	{
		println!("Grade = B");
	}  
	else if a >= 50.0 && a <= 59.0
	{
		println!("Grade = C");
	}
	else if a >= 45.0 && a <= 49.0
	{
		println!("Grade = D");
	}
	else if a >= 0.0 && a <= 44.0
	{
		println!("Grade = F");
	}

}