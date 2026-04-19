
// use std::io;
// use std::io::Write;
// This can combined since they share common module
use std::io::{self, Write};

// If we want to bring all public items into scope, we can use * glob operator
// E.g. use std::collections::*;
// Mostly used to bring everything into scope when writing tests


// Declaration of a module
// By default everythin define inside is private
mod front_of_house;

mod back_of_house;


// For functions, it is good practice to just bring the parent module into scope
// So when we use the function we have to specify the parent module
// Which indicates the function is not define locally
// E.g. use crate::front_of_house::hosting;
// But with structs, enums and other items, good practice to specify the whole path
// Except if we have to bring two items with same name
// Then having to specify the parent module helps
use crate::back_of_house::Appetizer;
// Anohther solution to the name conflict is to use the as keyword
// E.g. use crate::back_of_house::Breakfast as breakfast;

// We can also make the imported module public with the public keyword
// This is called re-exporting
pub use crate::front_of_house::hosting;

fn deliver_order() {}

// If we moved this to another module, we also move the above use into the same module
pub fn eat_at_restaurant() {
    // We have something called module tree when declaring modules
    // crate is the root for this which is implicit

    // Absolute path
    // let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    // Relative path
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I would like {} toast please", meal.toast);

    let appetizer = Appetizer::Soup;

    println!("I would like {:?} appetizer please", appetizer);

    // With use we can bring modules to scope and use them directly
    hosting::add_to_waitlist();
}
