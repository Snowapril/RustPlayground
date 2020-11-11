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
}
