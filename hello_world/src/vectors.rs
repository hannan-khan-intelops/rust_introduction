use std::mem;

pub fn run() {
	// array length of 5
	let mut numbers: Vec<i32> = vec![1,2,3,4,5];
	
	// re-assign
	numbers[2] = 22;
	
	println!("{:?}", numbers);
	
	//add on to the vector
	numbers.push(5);
	numbers.push(6);
	
	//pop off a number at the last position
	numbers.pop();
	
	// get single value
	println!("single value = {}", numbers[0]);
	
	// get array length
	println!("vector.len == {}", numbers.len());
	
	// arrays are stack allocated
	println!("vector occupies {} bytes.", mem::size_of_val(&numbers));
	
	// get slice from array
	let slice: &[i32] = &numbers[1..3];
	println!("slice: {:?}", slice);
	
	//loop through vector values
	for x in numbers.iter() {
		println!("Number: {}", x);
	}
	
	// mutate while looping
	for x in numbers.iter_mut() {
		*x = *x*2;
	}
	println!("Numbers vec {:?}", numbers);
}