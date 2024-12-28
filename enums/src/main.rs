#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        } 
    }
}

fn plus_one(value: Option<i32>) -> i32 {
    match value {
        Some(x) => x + 1,
        None => 1,
    }
}

fn main() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4(String),
        V6(String),
    }
    
    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    
    let home = IpAddr {
        kind: IpAddrKind::V4(String::from("V4")),
        address: String::from("127.0.0.1"),
    };

    println!("Home: {home:?}");

    let four = IpAddrKind::V4(String::from("V4"));
    let six = IpAddrKind::V6(String::from("V6"));

    println!("{four:?}");
    println!("{six:?}");

    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    let penny = Coin::Penny;

    println!("{alabama_quarter:?}");
    println!("{penny:?}");

    println!("Alabama quarter price: {}", value_in_cents(alabama_quarter));
    println!("Penny price: {}", value_in_cents(penny));

    println!("{}", plus_one(Some(25)));
    println!("{}", plus_one(None));

    let ten = Some(10);

    if let Some(value) = ten {
        println!("The value is: {}", value);
    } 
}
