pub fn run() {
	let name = "Brad";
	let mut age = 37;
	
	age = 38;
	
	println!("My Name is...{} and I am {}", name, age);
	
	
	// define constant
	const ID: i32 = 001;
	println!("ID: {}", ID);
	
	// assign multiple vars at once
	let (my_name, my_age) = ("Brad", 37);
	println!("My Name is...{} and I am {}", my_name, my_age);
}