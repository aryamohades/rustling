const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("three hours in seconds {THREE_HOURS_IN_SECONDS}");
    let mut x = 5;
    println!("value of x is {x}");
    x = 6;
    println!("value of x is {x}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("value of x in the inner scope is {x}");
    }
    println!("value of x {x}");

    let sum = 5 + 3;
    println!("sum: {sum}");

    let product = 5 * 3;
    println!("product {product}");

    let quotient = 5 / 3;
    println!("quotient: {quotient}");

    let truth = true;
    println!("truth: {truth}");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    another_function();
    func_with_parameters(25);

    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {y}");

    let x = five();
    println!("x: {x}");

    let y = plus_one(1);
    println!("y: {y}");
}

fn another_function() {
    println!("Another function.");
}

fn func_with_parameters(x: i32) {
    println!("x: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
