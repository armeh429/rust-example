enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


// Option<T> is a generic enum that is defined by the standard library  
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


//Catch-all Patterns and the _ Placeholder
fn add_fancy_hat() {
    println!("A fancy hat was added!");
}

fn remove_fancy_hat() {
    println!("A fancy hat was removed!");
}


fn main() {
    let coin = Coin::Dime;
    let value = value_in_cents(coin);
    println!("The value of the coin is: {} cents", value);

// Option<T> is a generic enum that is defined by the standard library
    let five = Some(5);
    let six = plus_one(five);   
    let none = plus_one(None);
    println!("six: {:?}", six);
    println!("none: {:?}", none);


//Catch-all Patterns and the _ Placeholder
    let dice_roll = 3;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), 
    }
}
