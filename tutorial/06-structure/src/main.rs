// Note that the entire instance must be mutable
struct User {
    username : String,
    email : String,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        username: String::from("sinjihng"),
        email: String::from("sinjihng@naver.com"),
        active: true,
        sign_in_count: 1
    };
    println!("Username is {}", user1.username);
    
    let mut user2 = User {
        username: String::from("snowapril"),
        email: String::from("sinjihng@gmail.com"),
        active: true,
        sign_in_count: 1
    };
    user2.username = String::from("sinjihng");

    let user3 = build_user(String::from("sinjihng@pusan.ac.kr"), String::from("sinjihng"));

    let user4 = User {
        email: String::from("sinjihng@naver.com"),
        username: String::from("sijihng"),
        ..user3
    };

    // tuple struct : don't have names associated with their fields
    // useful when you want to give the whole tuple a name and make the tuple be a different type from other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black({0}, {1}, {2})", black.0, black.1, black.2);

    // Unit like structures because they behave similarly to (), the unit type
    
}

//Using the Field Init Shorthand when Variables and Fields Have the Same Name
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count : 1,
        active : true
    }
}