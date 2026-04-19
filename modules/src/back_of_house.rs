// Here, we make this struct public but its fields are private by default
pub struct Breakfast {
    // We need to use pub to make the fields public too
    pub toast: String,
    seasonal_fruit: String
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("Peach")
        }
    }
}

// In contrast, if we declare an enum public, all its variant also public by default
#[derive(Debug)]
pub enum Appetizer {
    Soup,
    Salad
}

fn cook_order() {}

fn fix_incorrect_order() {
    cook_order();

    // Here, super is like .. in filesystem
    super::deliver_order();
}
