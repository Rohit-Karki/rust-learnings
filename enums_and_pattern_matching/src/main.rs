enum Coin{
    Penny,
    Nickel,
    Dime,
    Digital,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
            },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        Coin::Digital => 12,
    }
}

fn plus_one(x: Option<u8>) -> Option<u8>{
    match x{
        None => None,
        Some(x) => Some(x+1)
    }
}


enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
    println!("Hello, world!");
}
