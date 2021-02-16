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


    let mut s = String::new();

    let data = "initial contents";
    // to_string method which is available on any type that implements the Display trait.
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    // String is UTF8 encoded.
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("foo");
    let s2 = "bar";
    // push_str does not care about ownership of the parameter.
    // thus s2 is still available after calling push_str.
    s.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    // concatenation with the + operator or the format! macro.
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // note that ownership of the s1 is moved here and no longer be used.
    // println!("s1 is not available {}", s1);
    println!("s1 + s2 is {}", s3);

    // + operator use add method which looks like below.
    // fn add(self, s: &str) -> String {}
    // s1 is self here and &s2 is &String
    // compiler do coerce the &String into &str.
    // when we call add method, rust uses `deref coercion`
    // which turns &s2 into &s2[..]

    // copy is occurred in add method once.
    // s1 is move ownership to the function 
    // `copy` contents of the s2 
    // return ownership of the result.

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s1 + s2 + s3 is {}", s);
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format! macro does not take ownership of any of its parameter.
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s1 + s2 + s3 is {}", s);


    // indexing into Strings
    let s1 = String::from("Hello");
    //let h = s1[0]; this is an error
    //Rust string does not support indexing'

    // string internal representation (interpretation)
    // String is a wrapper over a Vec<u8>
    let hello = String::from("Hola");
    // in above case, len will be 4 and the vector storing the "Hola" is 4 bytes.
    // each of letters takes 1 byte when encoded in UTF-8
    let hello = String::from("Здравствуйте");
    // in above case, let will be 24, not 12.
    // each unicode scalar value in that string takes 2 bytes of storage.
    // therefore an index into the string's bytes will not always correlate to a valid unicode scalar value.

    // how to look at string from Rust
    // Three ways. (bytes, scalar values, and grapheme clusters)
    
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    // type of the s here is &str
    println!("{}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }

    //Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library.

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 80);
    scores.insert(String::from("Blue"), 100);

    let teams = vec![String::from("Red"), String::from("Blue")];
    let scores = vec![80, 100];

    let mut team_scores : HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    let field_name = String::from("favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Now field_name and field_value variables are no more valid. ownership moved.

    let name = String::from("favorite color");
    let value = map.get(&name);
    match value {
        Some(val) => println!("Some({})", val),
        None => println!("None!")
    };
    
    for (key, value) in &team_scores {
        println!("{} : {}", key, value);
    }

    let mut scores = HashMap::new();

    // Overwriting a value in the HashMap
    scores.insert(String::from("Blue"), 10);
    println!("{:?}", scores);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only inserting a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(100);
    scores.entry(String::from("Red")).or_insert(50);
    println!("{:?}", scores);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // or_insert method on entry return **mutable reference**
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // Given a list of integers
    let mut intList = [1, 10, 8, 19, -38, 2, 3];

    // Query average value
    let mut total : f32 = 0.0;
    for val in intList.iter() {
        total += *val as f32;
    }

    total /= intList.len() as f32;
    println!("average of list is {}", total);

    // Sort the list
    intList.sort();

    let mid_index = (intList.len() as f32 / 2.0) as usize;
    let medial_val = intList.get(mid_index);
    match medial_val {
        Some(val) => println!("Median value is {}", val),
        None => (),
    };

    let mut intCount = HashMap::new();
    for value in intList.iter() {
        let count = intCount.entry(value).or_insert(0);
        *count += 1;
    }
}