fn main() {
    // Two string types in Rust
    // str: string slice which is part of the core language, mostly used as &str
    // String: provided by the std lib
    // String implmented as a wrapper around vector of bytes
    let s = String::new();

    // We can also use some initial values
    let mut s = "Some string".to_string();
    // Same as
    // let str_lit = "Some string";
    // let mut s = str_lit.to_string();
    // let mut s = String::from("Some string");

    println!("Before: {s}");
    let extra_s = "Extra string";
    // push_str() takes &str i.e. slice of string as a reference
    s.push_str(extra_s);
    println!("Added: {s}");
    println!("After: {s}");

    // We can also use +
    let s_to_add = String::from("Added with +");
    let new_s = s + &s_to_add;
    // hte add is defined as add(self, &str) -> String
    // Hence, s is no longer valid after here since it is moved to new_s
    // Also the reason why we use reference of s_to_add
    // Also why we can use &String in place of &str is that the
    // compiler can coerce &String to &str
    // Here, &s_to_add is coerced to &s_to_add[..]
    println!("Added: {s_to_add}");
    println!("After: {new_s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // We can also use format! to concatenate strings
    // Does not take reference of any parameters
    let concat_s = format!("{s1}-{s2}-{s3}");
    println!("{concat_s}");

    string_indexing();
}

fn string_indexing() {
    // String indexing is not valid in Rust
    // String are implemeted as vec<u8>
    // Fine with Latin chars
    // But with Cyrillic letter, requires two bytes to encode a single char
    // Even with Latin char indexing returns the value and not the char

    // We can use string slice instead of indexing
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // Here s contains the first two char since each char is encoded with 2 bytes
    println!("{s}");

    // While iterating over strings, we can be explicit about whether we want char or bytes
    for c in s.chars() {
        // Though not possbile with some scripts like Devanagari script
        println!("{c}")
    }
    for c in s.bytes() {
        println!("{c}")
    }
}

fn utf_8_encoded() {
    // Since strings are utf-8 encoded, all of the following are valid
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שלום");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

}
