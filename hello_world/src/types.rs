pub fn run() {
	// Default is "i32"
	let x = 1;
	
	// default is "f64"
	let y = 2.5;
	
	// add explicit type
	let z: i64 = 12390485712345;
	
	// find max size
	println!("Max i32 is {}", std::i32::MAX);
	println!("Max i64 is {}", std::i64::MAX);
	
	// boolean
	let is_active: bool = true;
	
	// boolean from expression
	let is_greater: bool = 10 > 5;
	
	// character (single-quotes)
	let a1 = 'a';
	let face = '\u{1F600}';
	
	println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}