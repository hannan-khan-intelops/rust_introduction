use std::mem;

pub fn run() {
	// array length of 5
	let mut numbers: [i32;  5] = [1,2,3,4,5];
	
	// re-assign (cannot add on, only change)
	numbers[2] = 22;
	
	println!("{:?}", numbers);
	
	// get single value
	println!("single value = {}", numbers[0]);
	
	// get array length
	println!("numbers.len == {}", numbers.len());
	
	// arrays are stack allocated
	println!("array occupies {} bytes.", mem::size_of_val(&numbers));
	
	// get slice from array
	let slice: &[i32] = &numbers[1..3];
	println!("slice: {:?}", slice);
}