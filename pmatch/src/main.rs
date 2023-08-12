enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
fn main() {
    let nickel = value_in_cents(Coin::Nickel);
    let dime = value_in_cents(Coin::Dime);
    let penny = value_in_cents(Coin::Penny);
    let quarter = value_in_cents(Coin::Quarter(UsState::Alabama));
    let quarter_alaska = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Nickel: {nickel}, dime: {dime}, penny: {penny}, quarter(Alabama): {quarter}, quarter(Alaska): {quarter_alaska}");

    println!();
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    println!();
    let dice_roll: u8 = 9;
    match dice_roll {
        3 => println!("add_fancy_hat"),
        7 => println!("renive_fancy_hat"),
        other => println!("move_player {other}"),
    };
    match dice_roll {
        3 => println!("add_fancy_hat"),
        7 => println!("renive_fancy_hat"),
        _ => println!("igrnored without use value, roll again/reroll"),
    };
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Quarter(state) => {
            println!("state quarter from: {:?}", state);
            25
        }
        // Coin::Quarter(UsState::Alaska) => 30,
        // Coin::Quarter(UsState::Alabama) => 30,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
