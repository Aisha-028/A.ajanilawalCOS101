use std::io;

fn character(){

	let mut input = String::new();
	println!("Enter a character:");
	io::stdin().read_line(&mut input).expect("Failed to read input");
	let ch:char = input.trim().parse().expect("Invalid input");

	If ch:char >= '0' && ch <= '9'
	{
		println!("Character{}", );
	}
}