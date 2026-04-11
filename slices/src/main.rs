fn main() {
    let str = String::from("Hello World");

    let first_word = first_word_slice(&str);

    // str.clear(); -> gives error if first_word is used later
    println!("The first word is {first_word}");

    // This explains string literals for Rust as they are stored in the program binary
    // We just take their slice as immutable reference
    let str_literal_slice = "Hello Universe";

    let _ = first_word_by_slice(&str[..]);
    let _ = first_word_by_slice(&str[..5]);

    let _ = first_word_by_slice(&str_literal_slice[..]);
    let _ = first_word_by_slice(&str_literal_slice[..5]);

    // Also, str is string slice type but it cannot be stored by itself
    // Always used with reference or pointer

    // Other type slice, &[T] or &mut [T]
    let arr = [1, 2, 3, 4, 5];
    let second_and_third = &arr[1..3];
    assert_eq!(second_and_third, &[2, 3]);

    // range as x..y is y-exclusive
    // For y-inclusive we use x..=y
}

// This is better version since it works with both literals and String now
fn first_word_by_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
            // here 0 can be omitted as s[..i]
        }
    }

    &s[..]
    // &s[0..s.len()]
    // &s[..s.len()]
    // &s[0..]
}

// This way of doing this is tedious in many ways
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        // Here if I don't use &item above comparison is not possible between u8 and &u8
        // So I either take the reference above or derefence below as *item
        // Digging further, for does pattern matching here
        // So using item makes it &u8 while
        // using &item unwraps the &u8 and binds the u8 value to item
        // Kinda weird not gonna lie
        if item == b' ' {
            return i;
        }
    }

    s.len()
}