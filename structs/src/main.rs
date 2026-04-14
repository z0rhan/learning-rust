struct Order {
    id: u64,
    price: f64,
    size: u64,
    direction: i8
}

// Tuple struct that is tuple with a name
struct Color(u32, u32, u32);

// Unit [()] like struct
// Books says for implementing traits which will be covered later
struct AlwaysEqual;

fn main() {
    let mut sell1 = new_order(5647382910, 500.0, 100, 1);

    // let mut buy = Order {
    //     id: sell.id,
    //     price: 450.0,
    //     size: 50,
    //     direction: 2
    // };
    // Instead of above we can use the update syntax
    let mut sell = Order {
        price: 490.0,
        ..sell1 // Only this should come at last
    };
    // Also this moves sell1 to sell and sell1 is unusable after this
    // But here since all the integer and floating types implement Copy
    // There is copy here instead of move
    println!("The price of sell1 is {}", sell1.price);
    println!("The price of sell is {}", sell.price);

    let red = new_color(255, 0, 0);
    println!("The red, green and blue value of red is {} {} {}",
             red.0, red.1, red.2);
    // Unpacking the value requires the type to be mentioned unlike tuples
    let Color(r, g, b) = red;
    
    // Also the example uses String and not &str for the purpose of owning the data
    // Otherwise we have to specify the lifetime of the field, which will come later
}

fn new_order(id: u64, price: f64, size: u64, direction: i8) -> Order {
    Order {
        // Order of field does not matter
        direction,
        size,
        price,
        id
    }
}

fn new_color(r: u32, g: u32, b:u32) -> Color {
    Color(r, g, b)
}
