pub fn run() {
	let age: u8 = 30;
	let check_id: bool = true;
	let knows_person_is_of_age : bool = true;
	
	// if-else
	if age >= 21 && check_id || knows_person_is_of_age {
		println!("Bartender: you good.");
	} else if age < 21 && check_id {
		println!("Sorry you have to leave.");
	} else {
		println!("I'll need to see your ID.");
	}

	// shorthand if
	let is_of_age = if age >= 21 {true} else { false };
	print!("is_of_age == {}", is_of_age);
}