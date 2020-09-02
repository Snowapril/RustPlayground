fn main() {
    // single line Comment
    /*
    multi line comment.
    */

    // {} is a placeholder.
    println!("Rust says {}", "hello world");

    // scalar types : integer, floating-point, boolean, character
    let a = 10;
    let b = "sdfa";
    let c = 'k';
    println!("a : {}, b : {}, c : {}", a, b, c);

    let result = 10;
    let age:u32 = 20;
    let sum:i32 = -10;
    let mark:isize = 10;
    println!("result : {}, age : {}, sum : {}, mark : {}", result, age, sum, mark);
}