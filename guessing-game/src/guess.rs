pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(num: i32) -> Result<Guess, String> {
        if num < 1 || num > 100 {
            Err(String::from("Please choose a number between 1 and 100"))
        }
        else {
            Ok(Guess { value: num })
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
