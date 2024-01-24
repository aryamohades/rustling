fn main() {
    let s = String::from("hello, world!");
    let i = first_word(&s);
    println!("i: {i}");

    // String slice: reference to a part of a String.
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("message: {hello} {world}");

    let s = String::from("hello world");
    let first_word = first_word_slice(&s);
    println!("first word: {first_word}");

    let s = String::from("hello world");
    let s_literal = "hello world";

    // Following examples showcase the flexibility gained by using
    // &str as parameter type instead of &String. Both string literals
    // and String types can be used. Also makes use of "implicit deref coercions"
    // to coerce the type of &String into &str.

    // can pass reference to String type
    first_word_slice(&s);

    // can pass partial slice of String
    first_word_slice(&s[1..5]);

    // can pass the string literal as-is
    first_word_slice(s_literal);

    // can pass reference
    first_word_slice(&s_literal);

    // can pass partial using slice syntax
    first_word_slice(&s_literal[0..2]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]
        }
    }
    &s[..]
}
