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
	
	another_function(100);
}

// function bodies contain statements and expressions
// rust is expression based language
// statement do not return value
// function definition is statement
fn another_function(x : i32) {
	let y = 6; // This is statement. 6 is expression
	// let x = (let y = 10);  this is error because let is statement.
	// In C or C++, x = y = 10 will be both lead to the value 6.
	let x = 3 + 5; //3 + 5 is expression
	// calling a function and macro is expression
	let y = {
		let x = 3;
		x + 10
	};
	println!("y is {}", y);

	let test = test_function();
	println!("test value is {}", test);

	let test = test_function2();
	println!("test value is {}", test);
	// Control flow
	let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
	}
	// Using too mach else if can clutter the code. 
	// match keyword will be good at here.

	let condition = true;
	// if block and else block must have same variable type.
	let number = if condition { 5 } else { 6 };
	println!("number is {}", number);

	// Loops.
	let mut counter = 0;
	let result = loop {
		counter += 1;
		if counter == 10 {
			break counter * 20;
		}
	};
	println!("Loop result is {}", result);

	let mut number = 3;
	while number != 0 {
		println!("number in while is {}", number);
		number -= 1;
	}

	let array = [ 1, 2, 3, 4, 5 ];
	let mut index = 0;
	while index < 5 {
		println!("The value of the array is {}", array[index]);
		index += 1;
	}

	for element in array.iter() {
		println!("The value of the array is {}", element);
	}

	for number in (1..4).rev() {
		println!("number is {}", number);
	}
}

fn test_function() -> i32 {
	100
}

fn test_function2() -> i32 {
	return 1010;
}