struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Point(i32, i32, i32);

#[derive(PartialEq)]
struct AlwaysEqual;

fn main() {
    // Create new instance of struct
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someuser@example.com"),
        sign_in_count: 1,
    };

    println!("userame: {0}", user1.username);

    // Changing struct field value
    user1.username = String::from("anotheruser123");
    println!("username: {0}", user1.username);

    // Field init shorthand
    let username = String::from("someuser123");
    let email = String::from("someuser@example.com");

    let user = User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    };

    // struct update syntax
    let user2 = User {
        active: false,
        ..user1
    };

    let origin = Point(0, 0, 0);

    let v1 = AlwaysEqual;
    let v2 = AlwaysEqual;
    let equal = v1 == v2;
    println!("equal: {equal}");

    let rect = Rectangle {
        width: 30,
        height: 20,
    };
    let area = area(&rect);
    println!("rect: {:?}", rect);
    println!("rect pretty: {:#?}", rect);
    println!("area: {area}");

    let scale = 2;
    let mut rect = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    // Debug takes ownership of an expression, so if we want to continue
    // using rect after the dbg! call, we need to use a reference to rect.
    dbg!(&rect);

    println!("{:?}", rect);

    println!("area method: {0}", rect.area());
    rect.double();
    println!("area after doubling: {0}", rect.area());

    let bigger_rect = Rectangle {
        width: 100,
        height: 75,
    };
    let smaller_rect = Rectangle {
        width: 80,
        height: 30,
    };
    let can_hold = bigger_rect.can_hold(&smaller_rect);
    println!("can hold: {can_hold}");

    let square = Rectangle::square(25);
    println!("square: {:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    // basic method example
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // example of mutating using self
    fn double(&mut self) {
        self.width = self.width * 2;
        self.height = self.height * 2;
    }

    // method with same name as one of the struct's fields
    fn width(&self) -> bool {
        self.width > 0
    }

    // method with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // non-method associated function, using 'Self' as alias
    // for 'Rectangle'
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
