use std::io::{self, Write};

/// Main entry point for the vector module
///
/// Asks for user input for size of the 
/// 2 vectors to be produced
pub fn vec_entry() {
	
	println!("Vector Maths.");
	let size: usize;

	// get size of vectors

	size = get_var("Enter size pf the vectors:".to_string());

	let mut u = vec![0.0; size];
	let mut v = vec![0.0; size];

	println!("Enter vector 'u': ");
	build_vector(u.as_mut_slice(), size, 'u');
	println!("Enter vector 'v': ");
	build_vector(v.as_mut_slice(), size, 'v');

	println!("\nAdding u + v:");
	add_vectors(u.as_mut_slice(), v.as_mut_slice(), size);
	println!("\nSubtract u - v:");
	sub_vectors(u.as_mut_slice(), v.as_mut_slice(), size);	

	if size == 3 {
		println!("\nCross product of u,v");
		cross_product(u.as_mut_slice(), v.as_mut_slice(), size);
	}

	println!("Inner product u ⋅ v:");
	inner_product(u.as_mut_slice(), v.as_mut_slice(), size);

	println!("Normalize u:");
	norm_vect(u.as_mut_slice(), size, 'u');
	println!("Normalize v: ");
	norm_vect(v.as_mut_slice(), size, 'v');
}

/// 
fn build_vector(vect: &mut [f64], size: usize, ch: char) {

	let mut input = String::new();
	let mut index: usize = 1;

	println!("\nVector {}:", ch);
	while index <= size {
		input.clear();

		println!("Enter element {}:", index);
		print!("> ");

		io::stdout().flush()
			.expect("Flush failed.");

		//read line
		io::stdin().read_line(&mut input)
			.expect("Failed to read line");

		vect[index-1] = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		index += 1;
	}
}

fn add_vectors(vect1: &mut [f64], vect2: &mut [f64], size: usize) {

	let mut w = vec![0.0; size];
	let mut i: usize = 0;
	while i < size {
		w[i] = vect1[i] + vect2[i];
		i += 1;
	}

	i = 0;
	print!("u + v = [ ");	
	while i < size {
		print!("{} ", w[i]);
		i += 1;
	}
	println!("]");
}

fn sub_vectors(vect1: &mut [f64], vect2: &mut [f64], size: usize) {

	let mut w = vec![0.0; size];
	let mut i: usize = 0;
	while i < size {
		w[i] = vect1[i] - vect2[i];
		i += 1;
	}

	i = 0;
	print!("u - v = [ ");	
	while i < size {
		print!("{} ", w[i]);
		i += 1;
	}
	println!("]");
}

fn cross_product(a: &mut [f64], b: &mut [f64], size: usize) {
	
	let mut w = vec![0.0; size];
	
	w[0] = a[1]*b[2] - a[2]*b[1];
	w[1] = a[2]*b[0] - a[0]*b[2];
	w[2] = a[0]*b[1] - a[1]*b[0];

	let mut i: usize = 0;
	print!("u × v = [ ");	
	while i < size {
		print!("{} ", w[i]);
		i += 1;
	}
	println!("]");

}

fn inner_product(u: &mut [f64], v: &mut [f64], size: usize) {

	let mut i: usize = 0;
	let mut tmp = vec![0.0; size];

	while i < size {
		tmp[i] = u[i] * v[i];
		i += 1;
	}

	i = 0;
	let mut add_v = 0.0;
	while i < size {
		add_v += tmp[i];
		i += 1;
	}

	if add_v == 0.0 {
		println!("u ⋅ v = {} is orthogonal.", add_v);
	} else {
		println!("u ⋅ v = {} is not orthogonal.", add_v);
	}

}

fn norm_vect(vect: &mut [f64], size: usize, ch: char) {

	let mut i = 0;
	let mut tmp = vec![0.0; size];

	while i < size {
		tmp[i] = vect[i].powi(2);
		i += 1;
	}

	i = 0;
	let mut add_v = 0.0;
	while i < size {
		add_v += tmp[i];
		i += 1;
	}

	println!("‖{}‖ = √{} -> {}", ch, add_v, (add_v as f64).sqrt());
}

fn get_var(s: String) -> usize {

	let tmp: usize;
	loop {
		println!("{}", s);
		print!("> ");

		io::stdout().flush()
			.expect("Flush failed.");

		let mut input = String::new();

		io::stdin().read_line(&mut input)
			.expect("Failed to read line");

		tmp = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		break;
	}

	tmp
}