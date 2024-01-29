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