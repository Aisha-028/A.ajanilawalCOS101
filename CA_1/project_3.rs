fn main() {
	println!("CODE:::: ITEM  ::::PRICE(₦)");
	println!(" L  ::::Laptop ::::550,000");
	println!(" M  ::::Monitor::::120,000");
	println!(" K  ::::Keyboard::::15,000");
	println!(" H  ::::Headset::::25,000");

	let L:f64 = 550_000.0;
	let M:f64 = 120_000.0;
	let N:f64 = 15_000.0;
	let H:f64 = 25_000.0;

    //input item code
	println!("\nPlease Enter your item code.");
	let mut item_code = String::new();
	    io::stdin()
	    .read_line(&mut item code)
	println!("Item Code: {}", item _code);

	//input quantity
	println!("\nPlease enter your preferred quantity");
	let mut quantity = String::new();
	    io::stdin()
	    .read_line(&mut quantity)
	println!("Quantity: {}", quantity);

	let t = item_code * quantity
	println!("TOTAL COST:{}", t);

	if t > 500_000.0
	{
		let fa = t - (t * (7.0 / 100.0))
		println!("Final amount = {}", fa);
	}
	else if t <= 500_000.0
	{
		let fa = t 
		println!("Final amount = {}", fa);
	}
}
