fn main() {
    // infinite_loop();

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result: {result}");

    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;

        loop {
            remaining -= 1;

            if remaining == 5 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
        }
        count += 1;
    }
    println!("count: {count}");

    let mut countdown = 3;
    while countdown > 0 {
        println!("countdown: {countdown}");
        countdown -= 1;
    }
    println!("LIFTOFF!");

    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("element: {element}");
    }

    for number in (1..=3).rev() {
        println!("number: {number}");
    }
    println!("LIFTOFF!");
}

fn infinite_loop() {
    loop {
        println!("Hello, world!");
    }
}
