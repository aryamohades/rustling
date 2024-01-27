use std::net::{IpAddr, Ipv4Addr};

enum IPAddrKind {
    V4,
    V6,
}

enum IPAddr {
    V4(String),
    V6(String),
}

enum IPAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move(i32, i32),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn check_move(&self) {
        match self {
            Self::Move(x, y) => {
                println!("move to ({x}, {y})")
            }
            _ => {
                println!("not a move")
            }
        }
    }
    fn special(&self) {
        match self {
            Self::Quit => {
                println!("quit")
            }
            Self::Move(0, 0) => {
                println!("reset position")
            }
            Self::ChangeColor(0, 0, 0) => {
                println!("reset color")
            }
            _ => println!("nothing special"),
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let four = IPAddrKind::V4;
    let six = IPAddrKind::V6;

    route(four);
    route(six);

    let home = IPAddr::V4(String::from("127.0.0.1"));
    let loopback = IPAddr::V6(String::from("::1"));

    let home = IPAddr2::V4(127, 0, 0, 1);
    let loopback = IPAddr2::V6(String::from("::1"));

    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

    let msg_quit = Message::Quit;
    msg_quit.special();
    msg_quit.check_move();

    let msg_adv_pos = Message::Move(1, 1);
    msg_adv_pos.special();
    msg_adv_pos.check_move();

    let msg_reset_pos = Message::Move(0, 0);
    msg_reset_pos.special();

    println!("nickel value: {}", coin_value(Coin::Nickel));

    let six = plus_one(Some(5));
    println!("six: {:?}", six);

    let some_num = plus_one(None);
    println!("some_num: {:?}", some_num);
}

fn route(ip_kind: IPAddrKind) {}

fn coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
