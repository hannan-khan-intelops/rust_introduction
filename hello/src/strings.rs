pub fn run() {
	let mut hello = String::from("Hello");
	
	// push char
	hello.push('W');
	
	// push str
	hello.push_str("orld");
	
	//capacity in bytes
	println!("{} capacity in bytes = {}", hello, hello.capacity());
	
	// is empty
	println!("is empty = {}", hello.is_empty());
	
	// contain substring
	println!("Contains 'World' = {}", hello.contains("World"));
	println!("Contains 'Worsd' = {}", hello.contains("Worsd"));
	
	//replace
	println!("{} replace with there = {}", hello, hello.replace("World", "There"));
	
	// loop through string by whitespace
	println!("Using a for loop ==");
	for word in hello.split_whitespace() {
		println!("{}", word);
	}
	
	// get length (for both types of strings.)
	println!("{} length = {}", hello, hello.len());
	
	//create a string with capacity
	let mut s = String::with_capacity(10);
	s.push('a');
	s.push('b');
	s.push('c');

	// assertion testing
	assert_eq!(3, s.len());
	assert_eq!(10, s.capacity());
	
	
	println!("{}", s);
}