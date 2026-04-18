enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    Custom(u8),
    Fake
}

fn value_in_cents(coin: &Coin) -> Option<u8> {
    // Match control flow similar to the one I know
    // Except with pattern matching
    // Interesting thing that I encountered few chapters ago
    // &T when matched with &var will bind whatever &T refers to to var
    match coin {
        &Coin::Penny => Some(1),
        &Coin::Nickel => Some(5),
        &Coin::Dime => Some(10),
        &Coin::Quarter => Some(25),
        &Coin::Custom(value) => {
            println!("This is a custom Coin");
            Some(value)
        },
        &Coin::Fake => None
        // For example, we had more Coin variants but had no need for their values
        // Then we could use the some variable to capture the all other variants
        // other => {
        //      do_something()
        // }
        // Some could be done with '_' which does not bind the value to anything
        // Unlike above where the rest of the types are bind to other
    }
}

fn main() {
    let penny = Coin::Penny;
    let value = value_in_cents(&penny);
    match value {
        Some(value) => println!("The value of penny is {value}"),
        None => println!("penny is fake coin")
    }

    let custom = Coin::Custom(35);
    let value = value_in_cents(&custom);
    match value {
        // Here, Some(T) matches Some(value)
        // Hence, whatever is stored in Some(T) as T is bind to value
        Some(value) => println!("The value of custom is {value}"),
        None => println!("custom is fake coin")
    }

    let fake = Coin::Fake;
    let value = value_in_cents(&fake);
    match value {
        Some(value) => println!("The value of fake is {value}"),
        None => println!("fake is fake coin")
    }
}
