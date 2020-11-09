//bring standard IO library into the prelude.
use std::io;

// main function entry point.
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

	// in Rust, variable is immutable by default.
	// String::new(). new is an associated function of the String type. (be called as static method in c++)
	
    let mut guess = String::new();

	// without "use std::io;", we must specify like "std::io::stdin". quite crummy.
	// & indicates that this argument is reference.
	// variables, reference are immutable by default.
	// for mutable reference, we need to use "&mut"
	// read_line function returns io::Result which is enumerations.	
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}