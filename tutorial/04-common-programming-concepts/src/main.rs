fn main() {
    let x1 = 5;
	// In rust, a variable is immutable by default.
	// x1 = 10; 
	let mut x2 = 10;
	x2 = 12;
	println!("x2 is {}", x2);
	// constants and immutable is different.
	// not allowed to use mut to constants
	const MAX_POINTS: u32 = 100_000; // 100_000 == 100000. fo readability.
	println!("MAX_POINTS is {}", MAX_POINTS);
	
	// variable shadowing.
	let x2 = 100; 
	
	// practical use case of shadowing.
	let spaces = "     ";
	let spaces = spaces.len();
	println!("#spaces {}", spaces);
	
	let guess: u32 = "42".parse().expect("NAN!");
	// In rust, integer is by default i32 type.
	let number = 100; 
	
	let mut number: u8 = 250;
	// number += 10; -> in debug mode, compile error occur. "attempt to compute `250_u8 + 10_u8` which would overflow"
	
	// In rust, floating point is f64 by default.
	let number = 32.103; 
	let number: f32 = 32.103;
	
	let t = true;

    let f: bool = false; // with explicit type annotation
	
	// In rust, char is 4bytes by default for representing Unicode Scalar Types.
	let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
	println!("{}", heart_eyed_cat);
	
	// Compound type, tuple.
	let tuple: (i32, f64, u8) = (500, 6.4, 1);
	let (x, y, z) = tuple;
	println!("y is {}", y);
	
	let middle = tuple.1;
	println!("tuple getter {}", middle);
	
	// Compound type, array.
	// In rust, array have fixed length.
	let arr = [1, 2, 3, 4, 5];
	
	let arr: [i32; 5] = [1, 2, 3, 4, 5];	
	let initialized_arr = [3; 5]; // == [3, 3, 3, 3, 3]
	println!("First element of the initialized_arr = {}", initialized_arr[0]);
	
	another_function();
}

fn another_function() {
	println!("Hello");
}