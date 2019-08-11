use std::io::{self, Write};

struct MatrixT {
	width: usize,
	height: usize,
	mat: Vec<f32>
}

pub fn mat_entry() {

	println!("\nMatrix module:");

    let mut input = String::new();

    loop {

    	input.clear();

    	println!("Matrix Menu:");
    	println!("************");
    	println!("\t(1) Single operations");
    	println!("\t(2) Multi-matrix");
    	println!("\t(3) Exit");
    	print!("> ");

    	io::stdout().flush()
    		.expect("Flush failed.");

    	io::stdin().read_line(&mut input)
			.expect("Failed to read line.");
		println!(" ");
		let input_trim = input.trim();

		match input_trim.parse::<i32>() {
			Ok(choice) => match choice {
				1 => single_menu(),
				2 => multi_menu(),
				3 => break,
				_ => println!("'{}' is not an option\n", choice)
			},
			Err(..) => println!("Invalid input: {}", input_trim),
		};
    }
    println!("Goodbye!");
}

fn single_op() {

	let width = get_var("Input column count:".to_string());
	let height = get_var("Input row count:".to_string());
	let mut input = String::new();

	let mut mat_a = MatrixT { width: width, height: height, mat: vec![0.0; width * height] };

	println!("Define width:");

	for x in 0..mat_a.width {
		for y in 0..mat_a.height {

			let element = x + y * mat_a.width;
			while element < mat_a.width*mat_a.height {
				input.clear();

				println!("Input for [{}][{}]", x,y);

				io::stdin().read_line(&mut input)
					.expect("Failed to read line");

				mat_a.mat[element] = match input.trim().parse() {
					Ok(num) => num,
					Err(_) => continue,
				};

				break;
			}
		}
	}

	print_matrix(&mut mat_a);

}

fn multi_op() {

}

fn multi_menu() {

}

fn print_matrix(matrix: &mut MatrixT) {

	for x in 0..matrix.width {
		for y in 0..matrix.height {
			let element = x + y * matrix.width;
			print!("{} ", matrix.mat[element]);
		}
		print!("\n");
	}

	io::stdout().flush()
    	.expect("Flush failed.");

}

fn single_menu() {

	println!("\nMatrix module:");

    let mut input = String::new();

    loop {

    	input.clear();

    	println!("Single Matrix Operations");
    	println!("************************");
    	println!("\t(1) Transpose");
    	println!("\t(2) Inverse");
    	println!("\t(3) Determinat");
    	println!("\t(4) Exit");
    	print!("> ");

    	io::stdout().flush()
    		.expect("Flush failed.");

    	io::stdin().read_line(&mut input)
			.expect("Failed to read line.");
		println!(" ");
		let input_trim = input.trim();

		match input_trim.parse::<u32>() {
			Ok(choice) => match choice {
				1 => transpose(),
				2 => inverse(),
				3 => only_determinat(),
				4 => break,
				_ => println!("'{}' is not an option\n", choice)
			},
			Err(..) => println!("Invalid input: {}", input_trim),
		};
    }	
}

fn transpose() {

}

///
fn inverse() {
	//let mut width = get_var("Input column count:".to_string());
	//let mut height = get_var("Input row count:".to_string());
	//let mut input = String::new();

	let width = 2;
	let height = 2;

	let mut matrix = make_matrix(width, height);
	let det: f32 = 1.0/determinat(&mut matrix);
	let mut inv_matrix = MatrixT { width: width, height: height, mat: vec![0.0; width * height] };


	// swap |ab| to |d-b|
	//      |cd|    |-ca| 
	// a -> tmp
	let mut tmp = matrix.mat[0+0*inv_matrix.width];
	// d -> a
	inv_matrix.mat[0+0*matrix.width] = matrix.mat[1+1*matrix.width];
	// tmp -> d
	inv_matrix.mat[1+1*matrix.width] = tmp;
	inv_matrix.mat[1+0*matrix.width] = matrix.mat[1+0*matrix.width] * -1.0;
	inv_matrix.mat[0+1*matrix.width] = matrix.mat[0+1*matrix.width] * -1.0;

	println!("Before:");
	print_matrix(&mut matrix);
	scalar_matrix_mult(det, &mut inv_matrix);
	println!("After:");
	print_matrix(&mut inv_matrix);
	println!("Check:");

	let mut res_mat = MatrixT { width: width, height: height, mat: vec![0.0; width * height] };
	matrix_matrix_mult(&mut matrix, &mut inv_matrix, &mut res_mat);
	print_matrix(&mut res_mat);
}


fn determinat(matrix: &mut MatrixT) -> f32 {

	let det: f32 =
	(matrix.mat[0 + 0 * matrix.width]*matrix.mat[1 + 1 * matrix.width]) -
	(matrix.mat[1 + 0 * matrix.width]*matrix.mat[0 + 1 * matrix.width]);

	det
}

/// Finds the determinat of a matrix
///
/// Currently expects a 2 X 2 matrix
///
/// Expect up to 4 X 4
fn only_determinat(){

	let mut matrix = make_matrix(2, 2);

	let det =
	(matrix.mat[0 + 0 * matrix.width]*matrix.mat[1 + 1 * matrix.width]) -
	(matrix.mat[1 + 0 * matrix.width]*matrix.mat[0 + 1 * matrix.width]);

	println!("For Matrix:");
	print_matrix(&mut matrix);
	println!("\nDeterminate is: {}\n", det);
}

fn make_matrix(col: usize, row: usize) -> MatrixT {

	let mut mat_a = MatrixT { width: col, height: row, mat: vec![0.0; col * row] };
	let size = mat_a.width * mat_a.height;
	let mut input = String::new();
	for x in 0..mat_a.width {
		for y in 0..mat_a.height {

			let element = x + y * mat_a.width;
			while element < size {
				input.clear();

				println!("Input for [{}][{}]", x, y);

				io::stdin().read_line(&mut input)
					.expect("Failed to read line");

				mat_a.mat[element] = match input.trim().parse() {
					Ok(num) => num,
					Err(_) => continue,
				};

				break;
			}
		}
	}

	mat_a
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

fn scalar_matrix_mult(scalar: f32, matrix: &mut MatrixT) {

	for x in 0..matrix.width {
		for y in 0..matrix.height {

			let element = x + y * matrix.width;
			matrix.mat[element] = scalar * matrix.mat[element];
		}
	}
}

fn matrix_matrix_mult(mat_A: &mut MatrixT, mat_B: &mut MatrixT, mat_result: &mut MatrixT) {

	// validate incoming matrix compatability

	let a:f32 = mat_A.mat[0+0*mat_A.width]*mat_B.mat[0+0*mat_B.width] +
									mat_A.mat[1+0*mat_A.width]*mat_B.mat[0+1*mat_B.width];
	let b:f32 = mat_A.mat[0+0*mat_A.width]*mat_B.mat[1+0*mat_B.width] +
									mat_A.mat[1+0*mat_A.width]*mat_B.mat[1+1*mat_B.width];
	let c:f32 = mat_A.mat[0+1*mat_A.width]*mat_B.mat[0+0*mat_B.width] +
									mat_A.mat[1+1*mat_A.width]*mat_B.mat[0+1*mat_B.width];
	let d:f32 = mat_A.mat[0+1*mat_A.width]*mat_B.mat[1+0*mat_B.width] +
									mat_A.mat[1+1*mat_A.width]*mat_B.mat[1+1*mat_B.width];

	mat_result.mat[0+0*mat_A.width] = a;

	mat_result.mat[1+0*mat_A.width] = b;

	mat_result.mat[0+1*mat_A.width] = c;

	mat_result.mat[1+1*mat_A.width] = d;


}