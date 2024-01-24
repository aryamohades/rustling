fn main() {
    let x = 3;
    if x < 5 {
        println!("condition is true");
    } else {
        println!("condition is false");
    }

    let condition = true;
    let x = if condition { 3 } else { 5 };
    println!("x: {x}");
}
