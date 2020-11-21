#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// define method
impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // method
    fn can_hold(&self, other: Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    // associated functions. it is function not method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 100
    };
    
    println!("Area of the rectangle is {}", rect.area());

    // the specifier :? inside the curly brackets tells println! we want to use an output format called Debug
    println!("Area of the rectangle is {:#?}", rect);

    let squared_rect = Rectangle::square(10);
    println!("Squared rectangle info : {:#?}", squared_rect);
}