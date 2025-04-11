fn main() {

    // let four = IpAddKind::V4;
    // let six = IpAddKind::V6;

    // println!("four : {four:#?}, six : {six:#?}");

    // let home = IpAddr {
    //     kind: IpAddKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddKind::V6,
    //     address: String::from("::1"),
    // };


    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("this home : {home:#?}");
    println!("this loopback : {loopback:#?}");


    // option은 match와 if let 을 사용해 값을 추출할 수 있다.
    option_example();

    option_example_coin();
}

fn option_example_coin(){

    let dime = value_in_cents(Coin::Dime);
    let quarter = value_in_cents(Coin::Quarter(UsState::Alsaka));

    println!("this coin : Dime , {}", &dime);
    println!("this coin : Quarter , {}", &quarter);

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alsaka,
    }

    fn value_in_cents(coin : Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:#?}", state);
                25
            },
        }
    }

}

fn option_example(){

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number:Option<i32> = None;

    let x: i8 = 10;
    let y: Option<i8> = Some(5);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i+1),
        }
    }
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddrKind {
    // 각 값을 rust에서 valiant라고 부른다
    V4(u8, u8, u8, u8),
    V6(String),
}
