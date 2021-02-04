use rand::Rng;

// Nested paths
use std::{cmp::Ordering, io};
// use std::io::{self, Write};

// glob operator
use std::collections::*;

fn main() {
    println!("Hello, world!");
    
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is {}", secret_number);
}
