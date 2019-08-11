use std::io::{self, Write};

pub fn get_single<T>(s: String, arg: &mut T) {

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
