fn main() {
    let mut str: String = "Hello".to_string();

    let len = calculate_length(&str);
    // Here & allows us to create a reference to a variable without owning it

    println!("The length of {str} is : {len}");

    // Using mutable reference to change the value
    let mut_str: &mut String = &mut str;
    change_str_mut(mut_str);
    println!("The mutated string is : {str}");
    // But with mutable reference, only one reference is allowed at a time
    // We can have multiple immutable reference tho
    let mut_str1 = &mut str;
    println!("Second mut ref: {mut_str1}");
    // The above is okay tho, since reference scope ends where it is last used
    // println!("ref1: {mut_str} ref2: {mut_str1}") -> error
    // because now both mut ref scope end above and two mut ref cannot exist at once

    // let dang_str = dangling_ref(); -> error
    // The compiler enforces that there are not dangling ref

    let no_dang_str = no_dangle();
    println!("{no_dang_str}");
}

fn calculate_length(str: &String) -> usize {
    str.len()
} // Here, since s is a reference and does own the value
  // Drop is not called when s goes out of scope

fn change_str(str: &String) {
    // References are immutable by default so we cannot modify them
    // str.push_str(", world"); -> error
}

fn change_str_mut(str: &mut String) {
    // We can use mutable reference if we want to modify references
    str.push_str(", world!");
}

// This is not allowed by the compiler
// fn dangling_ref() -> &String {
//     let str = String::from("Dangling");
// 
//     &str
//     // After the function returns str goes out of scope and is Dropped
//     // But we are return a ref to it, which is now dangling ref
// }

fn no_dangle() -> String {
    let str = String::from("No Dangling str");

    str
    // Here now the ownership is returned to the caller
}