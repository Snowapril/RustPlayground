// define enumeration
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {

    }
}

fn route(kind: IpAddrKind) {

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello world"));
    m.call();

    let some_num = Some(10);
    let some_str = Some("sinjihng");
    let absent_num : Option<u32> = None;

    let x : i8 = 10;
    let y : Option<i8> = Some(10);
    // let sum = x + y; cannot do this.
    // error[E0277]: cannot add `std::option::Option<i8>` to `i8`
    
    let val = value_in_cents(Coin::Penny);
    let val2 = value_in_cents(Coin::Quarter(UsState::Alaska));

    // Same with default in swtich-case 
    let some_value = 0u8;
    match some_value {
        1 => println!("One"),
        3 => println!("Three"),
        5 => println!("Five"),
        _ => println!("Default")
    };

    // if let syntax
    // if some_value is matched with pattern "Some(3)"
    // do something. otherwise, do nothing
    let some_value = Some(0u8);
    if let Some(3) = some_value {
        println!("Three");
    } else {
        println!("Otherwise");
    };
    
}
