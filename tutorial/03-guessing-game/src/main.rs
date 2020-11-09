//bring standard IO library into the prelude.
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// main function entry point.
fn main() {
    println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret number is {}.", secret_number);
	
	loop 
	{
		println!("Please input your guess.");

		// in Rust, variable is immutable by default.
		// String::new(). new is an associated function of the String type. (be called as static method in c++)
	
    	let mut guess = String::new();

		// without "use std::io;", we must specify like "std::io::stdin". quite crummy.
		// & indicates that this argument is reference.
		// variables, reference are immutable by default.
		// for mutable reference, we need to use "&mut"
		// read_line function returns io::Result which is enumerations(Ok or Err)
		// If returned io::Result is ERR, string in expect function will be printed in crashed log.
	
    	io::stdin()
	        .read_line(&mut guess)
	        .expect("Failed to read line");
	
		// rust allow variable shadowing. guess is already used variable but we could redefine.
		// trim method eliminates any whitespaces at the beginning and end of the string.
		// parse method make string into number type which is type of the variable.
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("Please input the number only.");
				continue;
			}, // _ means catch all values
		};
	
		// {} is placeholder
	    println!("You guessed: {}", guess);
	
		// cmp method compares guess and secret_number and returns variant of the Ordering enum.
		// use match in order to decide what to do next based on the variant of the Ordering.
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small"),
			Ordering::Equal => {
				println!("Exactly!");
				break;
			} ,
			Ordering::Greater => println!("Too big")
		};
	};
	
    
}