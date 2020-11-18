//! memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.

//! Ownership Rules
// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.
fn main() {
    {
        let s = "hello";
    } // s is no longer valid.
    // println!("{}", s); impossible.

    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);

    let s2 = "sinjihng"; // string literal

    // Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope. 
    {
        let mut s = String::from("SNOWAPRIL");
    } // s is automatically deallocated because it goes out of scope.

    // assign 10 to x and copy the value of the x to y.
    let x = 10;
    let y = x; 

    // string is allocated in heap and it's pointer is stored in String class and save to s1.
    // "MOVE" s1 to s2. thus, s1 is no longer valid.
    let s1 = String::from("sinjihng");
    let s2 = s1;
    // println!("{}", s1); this is error.

    // Deep copy
    let s1 = String::from("sinjihng");
    let s2 = s1.clone();
    println!("s1 : {}, s2 : {}", s1, s2);

    takes_ownership(s1);
    // s1 is no longer valid now.

    let x = 5;
    makes_copy(x);
    println!("{}", x); // this is valid.

    let s1 = gives_ownership();
    println!("{}", s1);
    let s2 = String::from("world");
    let s3 = takes_and_gives_back(s2);
    // s2 is no longer valid.

    // When a variable that includes data on the heap goes out of scope, 
    // the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

    let (s4, len) = calculate_length_tuple(s3);
    println!("{} {}", s4, len);

    let len = calculate_length(&s4);
    println!("#{} : {}", s4, len);

    let mut s = String::from("Hello, ");
    change(&mut s);

    let mut s = String::from("hello");
    let r1 = &mut s;

    // you can have only one mutable reference to a particular piece of data in a particular scope. 
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s; //this is ok.
    // let r3 = &mut s; this is problem.
    // cannot borrow `s` as mutable because it is also borrowed as immutable
    
    let mut s = String::from("hello");

    // reference’s scope starts from where it is introduced and continues through the last time that reference is used
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // String slice
    // string slice is pointing to original string object.
    let mut s = String::from("hello world"); 
    let hello = &s[0..5]; // equivalent with &s[..5]
    let world = &s[6..11]; // equivalent with &s[6..]
    let entire = &s[0..s.len()]; // equivalent with &s[..];

    let word = first_word(&s);

    //s.clear(); error occurred here.
    // immutable borrow occurs in first_word(&s);
    // cannot do mutable borrow.

    // in borrowing rules, if we have an immutable reference, we cannot also take a mutable reference.
    // because clear needs to truncate string, it need mutable reference. -> compile fails.

    let s = "Hello, world"; // string literals are string slice (str)

    let my_string = String::from("Hello world");
    let word = first_word_str(&my_string[..]);
    let my_string_literal = "Hello world";
    let word = first_word_str(&my_string_literal[..]);
    let word = first_word_str(my_string_literal); // string literal is already string slice.

    // other slices.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[2..4]; // type of the slice is &[i32];
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: u32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

// we call having references as function parameters borrowing
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("world");
}
// string slice is written as str
fn first_word(s: &String) -> &str { 
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}