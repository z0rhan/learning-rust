use std::fmt::Display;

fn main() {
    let r;
    let x = 5; // lifetime of x starts here

    r = &x; // borrow of x starts here

    println!("{r}");

    let str1 = "Longer str";
    let str2 = "Shorter";

    let result = longest(str1, str2);
    println!("The longer string is {result}");


    let res_view = StringView {
        string: result
    };
    res_view.print();

    println!("The first word of result is :{}", first_word(&result));


    // Special lifetime -> static
    // Lives as long as the program, all string literal have static lifetime
    let static_str: &'static str = "Some string literal";
}

// This function can return reference to either str1 or str2 depending on the
// if else branch
// But the problem is the compiler cannot to borrow check on the returned
// value like it does normally
// So now we have to add generic lifetime parameters
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    // Here 'a takes the smaller of the lifetime of str1 and str2
    if str1.len() > str2.len() {
        str1
    }
    else {
        str2
    }
}

// Here 'a is a lifetime annotation
// They don't change the lifetime of the references itself
// Just provide information to the compiler about the lifetime of references


// Lifetime annotation is also needed when we define structs to hold references
struct StringView<'a> {
    string: &'a str
}

impl<'a> StringView<'a> {
    // Here the third rule from elision rules applies if the method have some
    // return value whose lifetime is tied to lifetime input parameter
    fn print(&self) {
        println!("{}", self.string)
    }
}

// Here we notice we don't need to specify lifetime parameters
// Lifetime elision rules
// First: For each parameter, a different lifetime parameter is attached
// Second: If there is exactly one lifetime parameter, it is applied to all 
//         output lifetime parameter
// Third:  If there are multiple lifetime parameters but one of them is self,
//         the lifetime of self is applied to all output lifetime parameters
fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &str[..i]
        }
    }

    &str[..]
}


// Summary of generic, trait bound and lifetime
fn longest_with_announcement<'a, T>(str1: &'a str,
                                    str2: &'a str,
                                    announcement: Option<T>) -> &'a str
where
    T: Display,
{
    if let Some(value) = announcement {
        println!("Announcement: {value}")
    }

    if str1.len() > str2.len() {
        str1
    }
    else {
        str2
    }
}
