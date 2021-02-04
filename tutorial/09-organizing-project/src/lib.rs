mod front_of_house;
mod back_of_house;

// below both are do same thing.
// use crate::front_of_house::hosting;
pub use self::front_of_house::hosting;

// it is idiomatic not to bring whole path of the function
// but it is for the struct, enums and the others.
use std::collections::HashMap;

pub fn eat_at_restaurant() {

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Path aliasing with use keyword
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}

// use std::fmt::Result;
// use std::io::Result as IOResult;

// fn function1() -> Result {

// }

// fn function2() -> IOResult<()> {

// }