fn main() {
    let mut v : Vec<i32> = Vec::new();
    // Rust provides macro like below.
    // create vector and holds the given value 
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    for e in v.iter() {
        println!("Element in the vector is {}", e);
    }

    let secondElement : &i32 = &v[1];
    println!("Second element is {}", secondElement);
    
    match v.get(2) {
        Some(third) => println!("Third variable is {}", third),
        None => println!("There is no third element")
    };

    // This cause program crash
    // let does_not_exist = &v[100];

    // This does not case program crash.
    // Just return None.
    let does_not_exist = v.get(100);

    let first = &v[0];
    // Below codes are error.
    // As mutable and immutable references cannot be in the same scope.
    // "let first = &v[0]" cause borrowing.
    //v.push(100);
    //println!("The first element is : {}", first);

    let v = vec![41, 12 ,41 ,11,24];
    for i in &v {
        println!("Iterating v is : {}", i);
    }
    for (count, i) in v.iter().enumerate() {
        println!("Iterating v with index {} is : {}", count, i);
    }

    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i *= 10;
        println!("{}", i);
    }

    // Using an Enum for storing multiple types into vector.
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadSheetCell::Int(123),
        SpreadSheetCell::Float(14.14124),
        SpreadSheetCell::Text(String::from("Asdfdasfasdf"))
    ];

    for e in &row {
        match e {
            SpreadSheetCell::Int(value) => println!("int {}", value),
            SpreadSheetCell::Float(value) => println!("float {}", value),
            SpreadSheetCell::Text(value) => println!("text {}", value),
            _ => println!("Otherwise")
        };
    }
}
