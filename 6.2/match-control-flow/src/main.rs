enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter));
    println!("{}", value_in_cents(Coin::Penny));
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Nickel);

    value_in_cents_with_state(CoinState::Penny);
    value_in_cents_with_state(CoinState::Nickel);
    value_in_cents_with_state(CoinState::Dime);
    value_in_cents_with_state(CoinState::Quarter(UsState::Alaska));
    value_in_cents_with_state(CoinState::Quarter(UsState::Alabama));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // Catch all other patterns by using placeholder _ or _other.
        _ => reroll(),
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", six.unwrap());
    println!("{}", none.unwrap());
}

fn reroll() {
    println!("reroll");
}

fn remove_fancy_hat() {
    println!("remove fancy hat");
}

fn add_fancy_hat() {
    println!("add fancy hat");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => lucky(),
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn lucky() -> u8 {
    println!("Lucky penny!");
    1
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum CoinState {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents_with_state(coin_state: CoinState) -> u8 {
    match coin_state {
        CoinState::Penny => 1,
        CoinState::Nickel => 5,
        CoinState::Dime => 10,
        CoinState::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // match arms’ patterns must cover all possibilities.
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
