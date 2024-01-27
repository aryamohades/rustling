fn main() {
    // create new scope
    {
        // declare s
        let s = "hello";
        // s is valid here and until end of this scope
    }
    // scope is over; s is no longer valid

    // mutate string
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s: {s}");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1:s2 {s1}:{s2}");

    let s = String::from("hello");
    takes_ownership(s); // s is moved; it can no longer be used after this point
    // println!("s: {s}"); // uncomment to see compiler error trying to use s after move

    let x = 5;
    makes_copy(x);

    let s = gives_ownership();
    println!("given s: {s}");

    let s = String::from("hello");
    let s2 = takes_and_returns(s); // s is returned from the function; can continue using s after this line
    println!("returned s: {s2}");

    // Tedious example of passing a value while still being able to use it after
    // Achieved by using a tuple return and re-capturing the returned value.
    // There's a better way to handle this common scenario in Rust: references.
    let s = String::from("hello");
    let (s2, len) = calc_len(s);
    println!("s2:len {s2}:{len}");

    // Following example uses a fixed size array, which is stack allocated.
    // So when assigning b to a, a is actually copied, not moved. Therefore,
    // it's safe to continue using both a and b. If a had an unknown size at
    // compile-time (e.g. if a was a vector), it would need to be allocated
    // on the heap and a would be a pointer to where the value lives in the heap.
    // In this case, assigning b to a would cause a to be moved and become invalid.
    let a = [1, 2];
    let b = a;
    println!("{:?}", a);
    println!("{:?}", b);
}

fn takes_ownership(s: String) {
    println!("s: {s}");
    // s is dropped after function ends; the memory is freed
}

fn makes_copy(x: i32) {
    println!("x: {x}");
    // x goes out of scope; nothing special happens
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_returns(s: String) -> String {
    s
}

fn calc_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
