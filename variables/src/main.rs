fn main() {
    // Default immutable, can be made mutable with mut
    let x = 6;
    println!("The value of x is {x}");

    let mut y = 7;
    println!("The value of y is {y}");
    y = 8;
    println!("The value of y is {y}");

    // Always immutable
    const PI: f32 = 3.1415;
    println!("The value of pi is {PI}");

    // Shadowing
    let x = x + 1;
    println!("The shadowed value of x is {x}");
    {
        let x = x * 2;
        println!("The shadowed value of x within another scope is {x}");
    }
    println!("The shadowed value of x is {x}");

    // Shadowing can change the type of the variable too
    let name = "Harry";
    println!("This is what is stored in <name>: {name}");
    let name = name.len();
    println!("This is what is stored in <name>: {name}");
}
