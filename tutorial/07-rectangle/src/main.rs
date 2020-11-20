#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 100
    };
    
    println!("Area of the rectangle is {}", area(&rect));

    // the specifier :? inside the curly brackets tells println! we want to use an output format called Debug
    println!("Area of the rectangle is {:#?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
