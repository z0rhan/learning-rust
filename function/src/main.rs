

// Entrypoint to our program
fn main() {
    another_func();
    func_with_parameter(25);
    func_with_two_parameter(25, 'A');

    // Statements
    // Perform some action and do not return a value
    let y = 6; // Statement
    // Function definitions are also a statment
    // Though calling a functions are not
    // let y = (let x = 5); -> not valid as statment do not return a value

    // Expressions
    // Evaluate to a resultant value
    // Can be part of a statement
    // Calling a function is a statement
    let y = 6; // 6 is an expression that evaluates to 6
    // Calling a function is a expression
    let y = {
        let x = 3;
        x + 1 // This means the new scope block evaluates to 4
        // let x = x + 1; -> If this was used it would mean the expression i.e.
        //                   the new scope block evaluates to () as learned from previous section
    }; // A new scope block is also an expression, WEIRD
    // Expressions do not end with semicolon ';'
    // Expression when end with ';' turn to statment and do not return a value
    println!("The value of y is {y}");

    let y_square = square(y);
    println!("The square of y is: {y_square}");
}

// Rust only cares the function is defined in a scope exposed to the caller
fn another_func()
{
    println!("This is another_func()");
}

// Type of parameter must be specified
fn func_with_parameter(x: i32)
{
    println!("This is func_with_parameter()");
    println!("This was provided with parameter x: {x}");
}

fn func_with_two_parameter(x: i32, y: char)
{
    println!("This is func_with_two_parameter()");
    println!("This was provided with parameter x: {x}");
    println!("This was provided with parameter y: {y}");
}

fn square(x: i32) -> i32
{
    x * x
    // Functions implicity return last expression
    // Can return early using the 'return' keyword
    // return x * x;
    // Also there is not implicit conversion here unlike C/C++
}
