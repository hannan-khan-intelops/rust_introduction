pub fn run() {
	// print to console
	println!("Hello World! from print.rs");
	
	// basic formatting
	println!("{} is from {}", "John", "Utah");
	
	// positional arguments
	println!("{0} is from {1} and {0} likes to {2}", "John", "Utah", "CODE");
	
	// named arguments
	println!("{name} likes to play {activity}.", name="John", activity="baseball");
	
	// placeholder traits
	println!("Binary: {:b} Hex: {:x} Octal: {:o}", 11, 11, 11);
	
	// placeholder for debug trait
	println!("{:?}", (12, true, "hello"));
	
	// basic math
	println!("10 + 10 = {}", 10 + 10);
}