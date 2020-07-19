#[derive(Debug)]
enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let day1 = Days::Monday;
    let day2 = Days::Tuesday;
    println!("Hello, day1 is: {:?}", day1);
    println!("Hello, day2 is: {:?}", day2);
    println!("message of day {}", message_of_the_day(Days::Wednesday));
    println!("message of day {}", message_of_the_day(Days::Saturday));
    println!("message of day {}", message_of_the_day(Days::Sunday));
    println!("value of a Nickel: {}", value_in_cents(Coin::Nickel));
    println!(
        "value of a Quarter: {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );
}
fn message_of_the_day(day: Days) -> u8 {
    match day {
        Days::Saturday => {
            println!("what a lovely {:?}", day);
            5
        }
        Days::Sunday => {
            println!("who doesn't like {:?}", day);
            6
        }
        _ => {
            println!("I hate {:?} :)", day);
            day as u8
        }
    }
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
