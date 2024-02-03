use std::collections::HashMap;

fn main() {
    // Initialize new empty vector and declare type
    let vec: Vec<i32> = Vec::new();
    println!("{vec:?}");

    // Macro shortcut for creating new vector
    let vec = vec![1, 2, 3];
    println!("{vec:?}");

    let apple = Fruit {
        name: String::from("Apple"),
        calories: 50,
    };
    let banana = Fruit {
        name: String::from("banana"),
        calories: 75,
    };

    // Vector is generic.
    let vec = vec![apple, banana];
    println!("{vec:?}");

    // Create vector and push items into it.
    let mut vec = vec![1, 2, 3];
    vec.push(4);
    vec.push(5);
    println!("{vec:?}");

    // Accessing elements in the vector. 2 ways,
    // either by indexing or using the 'get' method.
    // The indexing syntax will panic if accessing an index out of bounds.
    // The get syntax returns an Option<T> as a safe way to access.
    // Which to use depends on your program.
    let first_element = vec[0];
    println!("first_element: {first_element}");

    // Following line will panic
    // let tenth_element = vec[10];

    // Safely access the vector at any index.
    let tenth_element = vec.get(9);
    match tenth_element {
        None => println!("vec does not have a tenth element"),
        Some(num) => println!("tenth element is: {num}"),
    };

    // Following will not compile because you are holding
    // a reference to an item and pushing to the vector, and
    // then attempting to use that reference later.
    // This is not safe because pushing to the vector may cause
    // items to move around in memory, invalidating that reference.
    // Rust will catch this potential bug at compile-time.
    // let item = &vec[0];
    // vec.push(6);
    // println!("item: {item}");

    // Iterate using mut ref and add 1 to each item.
    for item in &mut vec {
        *item += 1;
    }
    println!("{vec:?}");

    // Use iterator to move through the vector.
    let mut vec = vec![1, 2, 3];
    let mut iter = vec.iter();
    let next = iter.next().unwrap();
    println!("next: {next:?}");
    let next = iter.next().unwrap();
    println!("next: {next:?}");
    let next = iter.next().unwrap();
    println!("next: {next:?}");

    // Next call to iter.next().unwrap() will panic because
    // unwrap panics if there is no value to unwrap i.e. None.
    // let next = iter.next().unwrap();
    // println!("next: {next:?}");

    // Can also iterate over vector using a range.
    for i in 0..vec.len() {
        println!("index: {i}, value: {}", vec[i]);
    }

    // other common operations are supported e.g. pop
    let item = vec.pop().unwrap();
    println!("popped item: {item}");

    // We can store enum values in a single vector, even if the
    // enum variants themselves have different types. This is becase
    // only the enum itself is considered for typing purposes.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{row:?}");

    // Two ways to initialize a String
    let s = "hello".to_string();
    let s = String::from("hello");

    // Update string
    let mut s = String::from("hello");
    s.push_str(" world");

    // add single char to String
    s.push('!');

    let hello = String::from("hello");
    let world = String::from("world");
    // second arg of add (+) is a &str and first arg is consumed
    let hello_world = hello + &world;
    // Following line is invalid because hello was moved as a result of the + operation.
    // println!("{hello}");

    // Can use format! macro to quickly concatenate (also does not move anything because
    // it only uses references)
    let hello = String::from("hello");
    let world = String::from("world");
    let hello_world = format!("{hello} {world}!");

    let hello_world = String::from("hello world!");
    let hello = &hello_world[..5];
    println!("{hello}");

    // Iterate over string chars
    for c in hello_world.chars() {
        println!("{c}");
    }

    // Iterate over string bytes
    for b in hello_world.bytes() {
        println!("{b}");
    }

    // Create HashMap and insert a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // Access value unsafely; will panic if value doesn't exist
    let blue_score = scores["Blue"];

    // Access HashMap value using key. Using 'get' returns an Option type.
    let mut scores = HashMap::new();
    let team_name = String::from("Blue");
    scores.insert(team_name.clone(), 10);
    let score = scores.get(&team_name).copied().unwrap();
    println!("score: {score}");

    // Iterate over HashMap key-value pairs
    for (key, val) in &scores {
        println!("{key}: {val}");
    }

    // For owned values, the HashMap will become the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are now invalid; following will not compile
    // println!("{field_name}: {field_value}");

    // Updating HashMap: utility methods are provided to handle scenarios like:
    // overwriting a value
    // setting a value only if it doesn't exist
    // updating a value based on the old value if it exists

    // Basic replacement by setting same key multiple times.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    // Insert a value only if it doesn't exist yet using 'entry' and 'or_insert' methods.
    // 'or_insert' returns a mutable ref to the item if it exists. In this case, we ignore
    // the return value because we only want to insert if it doesn't exist, and do nothing
    // otherwise.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);

    // Update value based on old value.
    // Example: counting character frequency. We want to increment
    // the frequency, but first initialize to 0 if we haven't seen
    // the char yet. Again, 'or_insert' returns a mut ref to the item.
    let mut map = HashMap::new();
    let char = 'c';
    let count = map.entry(char).or_insert(0);
    *count += 1;
}

#[derive(Debug)]
struct Fruit {
    name: String,
    calories: u16,
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
