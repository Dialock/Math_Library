use std::io::{self, Write};

pub fn get_single_usize(s: String) -> usize {

	let ret_val: usize;

	loop {
		println!("{}", s);
		print!("> ");

		let mut input = String::new();

		io::stdout().flush()
			.expect("Flush failed.");

		io::stdin().read_line(&mut input)
			.expect("Failed to read line");

		ret_val = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		break;
	}

	ret_val
}

/*
pub fn get_single_i32(s: String) -> i32 {

	let ret_val: i32;

	loop {
		println!("{}", s);
		print!("> ");

		let mut input = String::new();

		io::stdout().flush()
			.expect("Flush failed.");

		io::stdin().read_line(&mut input)
			.expect("Failed to read line");

		ret_val = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		break;
	}

	ret_val
}

pub fn get_single_u32(s: String, arg: u32) {

	let ret_val

	loop {
		println!("{}", s);
		print!("> ");

		let mut input = String::new();

		io::stdout().flush()
			.expect("Flush failed.");

		io::stdin().read_line(&mut input)
			.expect("Failed to read line");

		arg = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		break;
	}
}

pub fn get_single_f32(s: String, arg: f32) {

	loop {
		println!("{}", s);
		print!("> ");

		let mut input = String::new();

		io::stdout().flush()
			.expect("Flush failed.");

		io::stdin().read_line(&mut input)
			.expect("Failed to read line");

		arg = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		break;
	}
}

pub fn get_single_f64(s: String, arg: f64) {

	loop {
		println!("{}", s);
		print!("> ");

		let mut input = String::new();

		io::stdout().flush()
			.expect("Flush failed.");

		io::stdin().read_line(&mut input)
			.expect("Failed to read line");

		arg = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		break;
	}
}
*/