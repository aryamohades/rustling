fn main() {
    let s = String::from("hello");
    let len = calc_len(&s);
    println!("s:len {s}:{len}");

    let mut s = String::from("hello");
    change_s(&mut s);
    println!("mutated s: {s}");

    // If you have a mutable ref to a value, there can be no other ref to that value.
    // Following won't compile due to this.
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &s;
    // println!("r1:r2 {r1}:{r2}");

    // Following is ok because the second reference is created after any usages
    // of the first mut reference.
    let mut s = String::from("hello");
    let r1 = &mut s;
    println!("r1: {r1}");

    let r2 = &s;
    println!("r2: {r2}");
}


fn calc_len(s: &String) -> usize {
    s.len()
}

fn change_s(s: &mut String) {
    s.push_str(", world!");
}

// This function does not compile because it returns a reference to s,
// but s is freed when the function ends, meaning that the reference to
// s would point to nothing in memory. Rust turns this potential runtime
// error into a compile-time error.
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// This is safe because ownership is moved out of the function
// and as a result, nothing is deallocated.
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
