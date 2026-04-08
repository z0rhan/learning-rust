// Each value in Rust has an owner
// There can only be one owner at a time
// If the owner goes out of scope, teh value will be dropped

fn main() {
    // s is not valid here, not declared yet
    {
        let _s = "hello"; // Valid from here
    }
    // scope is over and s is no longer valid
    memory_and_allocation();

    clone_heap_memory();

    let str: String = String::from("Hello");
    takes_ownership(str);
    // str is moved into the function and is invalid now

    let x = 5;
    makes_copy(x);
    // Since x implements Copy, the value is copied and x is still valid

    let mut str = String::from("Hello");
    
    // Likewise, return value can also move the value
    str = takes_and_gives_back(str);
    println!("{str}");

    let (str, len) = str_len(str);
    // This is what we would want everytime i.e. moving value into and back from the function
    // For this, Rust has references
    println!("Lenght of {str} is {len}");
}

fn memory_and_allocation()
{
    // This is stored as read only and the size is fixed
    let _s = "hello";

    {
        // String from a string literal
        let mut s = String::from("Hello");
        s.push_str(" world");

        println!("{s}");
    }
    // We allocated memory with the from() method
    // But after s goes out of scope the allocated memory is automatically dropped
    // Similar to RAII in C++
    // 'drop' is the special function which is called by the Rust compiler
    // Sounds like destructor for now

    let s1 = String::from("string1");
    let mut s2 = s1;
    // Here s2 gets the data pointer, size and capacity from s1
    // Now, since s1 and s2 both point to the same data, there can be issues
    // Hence, in Rust s1 is no longer valid
    // i.e. move here happened implicitly

    // println!("{s1}"); -> borrow of moved value
    println!("{s2}");

    s2 = String::from("string2");
    // Here the original string s2 was pointing to in the heap 'string1' is dropped too
    println!("{s2}");
}

fn clone_heap_memory()
{
    let s1 = String::from("string1");
    let s2 = s1.clone();
    // clone clones the memory in heap as well and s1 is not dropped
    println!("s1 = {s1}");
    println!("s2 = {s2}");

    let x = 5;
    let y = x;
    // Unlike with the heap data that are of unknown size at compile time
    // data type like int are fixed size and are on the stack
    // So they can be copied in the stack and there is no reason to make x invalid
    // Rust has Copy trait for data types with fixed size stored on stack
    // This Copy trait allows the data to be copied without moving
    // This trait cannot be implemented for type with Drop trait
    // As Drop trait means the res must be handled specially
    println!("x: {x}, y: {y}");
    // scalar types such as integer, bool, floating point and char implement Copy
    // Tuples with types that implement Copy can also implement Copy
}

fn takes_ownership(some_string: String) {
    println!("This is string is moved to the function: {some_string}");
}

fn makes_copy(integer: i32) {
    println!("This is a copy in the function: {integer}");
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("This is moved into the function: {some_string}");

    // moving the value by return
    some_string
}

fn str_len(some_str: String) -> (String, usize) {
    let len = some_str.len();

    (some_str, len)
}
