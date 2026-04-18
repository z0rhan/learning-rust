// Enumeration when we want to represent anything with fixed variants
enum IpAddrVer {
    V4(u8, u8, u8, u8),
    V6(String)
    // Might be tempted to use struct to store IpAddrVer and the address
    // But the address can be directly embedded with the variant itself
}

// struct IpAddr {
//     version: IpAddrVer,
//     addr: String,
// }

enum Message {
    Quit,
    MoveMessage{x: i32, y: i32},
    WriteMessage(String),
    ChangeColorMessage(i32, i32, i32)
}

// Like structs enums can also have associated functions
impl Message {
    fn call(&self) {
        println!("Message called")
    }
}

fn main() {
    let _home = IpAddrVer::V4(127, 0, 0, 1);
    let _loopback = IpAddrVer::V6(String::from("::1"));
    // Here the variant also acts as a constructor
    // IpAddrVer::V4() is a function that takes String and returns an instance of IpAddrVer
    let message = Message::WriteMessage(String::from("A message to write"));
    message.call();
}

fn option_type() {
    let _some_num = Some(5); // Option<i32>
    let _some_char = Some(5); // Option<char>
    let _some_i32: Option<i32> = None;

    // So this is basically an alt for null values
    // Guarantees that we dont use a value of type T when it is null
    // This is enforced by warpping it in the enum Option
    // Option<T> has either None or Some(T)
    // So we can either use T from Some(T), using methods of Option
    // or handle when it is None
    // For that we need match control flow
}
