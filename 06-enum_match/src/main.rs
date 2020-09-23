fn main() {
    enum_example_6_1_1();

    enum_example_6_1_2();

    enum_example_6_1_3();

    enum_example_6_1_4();
}

//////////////////////////     열거형과 패턴 매칭     //////////////////////////
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

fn enum_example_6_1_1 () {

    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

fn enum_example_6_1_2 () {
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    println!("{:?} / {:?} ", &home, &loopback);
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn enum_example_6_1_3 () {
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));
    println!("{:?} / {:?}", &home, &loopback);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 메소드 내용 정의
        self.Write
    }
}
fn enum_example_6_1_4 () {
    let m = Message::Write(String::from("hello"));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

////////

enum FunList<T> {
    Nil,
    Cons { head:  T, tail: Box<FunList<T>> }
}

impl FunList<T> {
    fn append(item: T) -> FunList<T> {
        match item {

        }
    }
}