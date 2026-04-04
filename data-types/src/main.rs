use std::str;

fn scalar_types()
{
    // Integer types
    // i8 - i128
    // u8 - u 128
    // isize and usize which depend on the platform arch

    // Deicmal, defaults to i32
    let _x = 98_222;
    // He_x
    let _x = 0xff;
    // Octal
    let _x = 0o77;
    // Binary
    let _x = 0b1111_0000;
    // Byte (u8) only
    let _x = b'A';

    // Floating point types
    // f32 and f64
    // Defaults to f64
    let _pi = 3.1415;

    // Numeric operations
    let _sum = 5 + 3;
    let _difference = 95.5 - 4.3;
    let _product = 4 * 3;
    let _quotient = 55 / 6;
    let _trucated = -5 / 3; // Truncates towards zero to the nearest integer
    let _remainder = 43 % 5;

    // Boolean types
    let _is_prime: bool = false;

    // Char type
    // char here is 4 bytes and can represent much more than just ASCII
    let _grade = 'c';
}


fn compound_types()
{
    // Typles
    // Group of different types, fixed length
    let tup: (i32, f32, u8) = (500, 4.8, 1);

    // Pattern matching to access individual values in a tuple
    let (x, y, z) = tup;
    println!("The first value in tup is {x}");
    println!("The second value in tup is {y}");
    println!("The third value in tup is {z}");

    // Index to access individual values in a tuple
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("The first value in tup is {x}");
    println!("The second value in tup is {y}");
    println!("The third value in tup is {z}");

    // Interesting thing: unit -> empty typle ()
    // Represents an empty value or empty return type
    // Expressions implicity return unit value if they don't return any other value
    let _unit: () = ();
    // println!("The value in unit is {unit}") -> () does not have the train std::fmt::Display

    // Array
    // Collection of same type, fixed length
    let _days: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
    // Here &str because string literal are stored somewhere and we are using ref to them

    let _a = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // This means an array of length 5 with all 3

    // Index to access elements of an array
    let _first = _a[0];
    let _second = _a[1];
    // let _out_of_bouds = _a[10]; -> Out of bounds access
}


fn main()
{
    // Use '_' prefix for unused variables
    compound_types();
}
