// Debug format to print out custom types
// Normally primitives use Display, which is not supported for custom types
// Debug is a trait, which we will cover later
#[derive(Debug)]
struct Rectangle {
    length: u32,
    breadth: u32
}

// Method
// Function but within the context of a struct or enum or trait object
// First parameter always 'self'
impl Rectangle {
    // Here self is alias for self: &Self
    // While Self is alias for the type impl block is for
    fn area(&self) -> u32 {
        self.length * self.breadth
        // Auto referencing/dereferencing
        // hence we dont need *self.length here
        // So, here ref &self is like self* in c/c++ but safer
        // Not like the reference in c/c++ at all
    }
    // methods can have same name as the fields

    fn can_hold(&self, other: &Rectangle) -> bool {
        if other.length > self.length {
            false
        }
        else if other.breadth > self.breadth {
            false
        }
        else {
            true
        }
        // Ngl, kinda like this last expression as return type style
    }
}

// It is valid syntax to have multiple impl blocks
// Though not any reason to seperate them here
impl Rectangle {
    // All function in the impl block are associated functions of the type of impl block
    // We can also define associated function that do not take self (not methods)
    // Usually used as constructors
    fn square(size: u32) -> Self {
        Self {
            length: size,
            breadth: size
        }
    }
}


fn main() {
    let rect1 = Rectangle {
        length: 40,
        breadth: 20,
    };

    // println!("Rectangle rect1 is {rect1:?}");
    // :? tells compiler to use the format called Debug
    // #:? is the prettier version
    println!("Rectangle rect1 is {rect1:#?}");

    let rect2 = Rectangle {
        length: dbg!(rect1.length * 2),
        breadth: 30
    }; // dbg! -> another way to print using the Debug format
       // Takes ownership and returns it
    dbg!(&rect2);

    println!("The area of rect1 is {}", rect1.area());

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1? {}", rect2.can_hold(&rect1));

    let square = Rectangle::square(20);
    println!("Rectangle square is {:#?}", square);
}
