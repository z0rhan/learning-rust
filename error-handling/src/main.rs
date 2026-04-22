use std::fs::{self, File};
use std::io::{self, ErrorKind};
use std::error::Error;

// main can also return Result<T, E>
fn main() -> Result<(), Box<dyn Error>>{
    let filename = "hello.txt";
    let greeting_file_result = File::open(filename);

    let greeting_file = match greeting_file_result {
        Ok(handle) => handle,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(new_handle) => new_handle,
                Err(create_error) => panic!("Failed to create file: {create_error:#?}")
            },
            _ => {
                panic!("Failed to open file: {error:#?}")
            }
        }
    };

    // We can do this in many ways
    // let greeting_file_result = File::open(filename).unwrap();
    // This returns value in Ok(value) else panics
    // let greeting_file_result = File::open(filename).expect("Error msg");
    // This allows to specify the message for panic

    let res = read_from_file(filename)
                .expect("File could not be opened");
    println!("The content in {filename} is {res}");

    // This does not exist
    let res = read_from_file("does_not_exist.txt")?;

    Ok(())
}

fn read_from_file(filename: &str) -> Result<String, io::Error> {
    // let file_result = File::open(filename);

    // let mut file = match file_result {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // let mut content = String::new();

    // match file.read_to_string(&mut content) {
    //     Ok(_) => Ok(content),
    //     Err(error) => Err(error)
    // }

    // The above can be simplied using ?
    // So ? just returns the value from Ok(value)
    // In case of error, returns Err(error) from the whole function
    //
    // let mut file = File::open(filename)?;
    // let mut content = String::new();
    // file.read_to_string(&mut content)?;
    // Ok(content)
    //
    // The main difference with ? is that it calls the from function for the error values
    // Defined in the From trait, it is used to convert one type to another
    // So, ? converts the error type received to the error return type of the function
    // For example we could have our own OwnError type
    // Then, by impl From<io::Error> for OwnError, ? converts all io::Error to OwnError
    // ? can also be used with Option<T> where None means early return
    // Else the expression evaluates to value of Some(value)

    // Ultimate shortened form with method chaining
    // let mut content = String::new();
    // File::open(filename)?.read_to_string(&mut content)?;
    // Ok(content)

    // Well std already has the method to do this
    fs::read_to_string(filename)
    // Lmao, what a journey
}
