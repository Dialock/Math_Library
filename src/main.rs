use std::io::{self, Write};

mod vectors;
mod matrices;

fn main() {
    println!("Welcome to Basic Linear Algebra Solver!");

    let mut input = String::new();

    loop {

    	input.clear();

    	println!("Options:\n\t(1) Vectors\n\t(2) Matrix");
    	println!("\t(3) Exit");
    	print!("> ");

    	io::stdout().flush()
    		.expect("Flush failed.");

    	io::stdin().read_line(&mut input)
			.expect("Failed to read line.");
		println!(" ");
		let input_trim = input.trim();
		//if input_trim == "Q" { break; }

		match input_trim.parse::<i32>() {
			Ok(choice) => match choice {
				1 => vectors::vec_entry(),
				2 => matrices::mat_entry(),
				3 => break,
				_ => println!("'{}' is not an option\n", choice)
			},
			Err(..) => println!("Invalid input: {}", input_trim),
		};

    }
    println!("Goodbye!");
}
