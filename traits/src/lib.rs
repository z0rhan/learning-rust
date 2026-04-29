use std::fmt::{Debug, Display};

pub trait Summary {
    fn author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}...", self.author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn author(&self) -> String {
        String::from(&self.author)
    }

}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub repost: bool
}

impl Summary for SocialPost {
    // This is only possible because we provide a default implementation
    // for the summarize() method of Summary trait
    // Otherwise we must provide implementation for summarize() here

    fn author(&self) -> String {
        String::from(&self.username)
    }
}

// Using traits to define functions that take multiple types
// pub fn notify(item: &impl Summary) {
// The above is just syntax sugar for trait bound
// The impl Trait is good when we allow different types like,
// pub fn notify(item1: &impl Summary, item2: &impl Summary) {
// But with trait bound it forces a sigle type unless we add another generic type
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
// We can do multiple trait bounds with +
// pub fn notify(item1: &(impl Summary + Display), item2: &impl Summary) {
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}


// For multiple generic types with multiple traits, the syntax gets mess
// pub fn some_func<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) {
// 
// }
// We can use where clause instead
pub fn some_func<T, U>(t: &T, u: &U)
where
    T: Clone + Display,
    U: Clone + Debug
{
 
}

// We can also use impl trait as a return type
// However the return expression in the function must be of a single type
// For e.g. it wouldn't work if we had if else statement which would return
// either a NewsArticle or SocialPost
pub fn returns_summarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("Some headline"),
        author: String::from("Some author"),
        content: String::from("Some content"),
    }
}


struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn new(x:T, y:T) -> Self {
        Self { x: x, y: y }
    }
}

// We can also use trait bounds to conditionally implement methods
// So it only works with Point with T that implements PartialOrd trait for
// comparison and Display trait for printing
impl<T: Display + PartialOrd> Point<T> {
    fn display_largest_field(&self) {
        if self.x > self.y {
            println!("The largest member is x: {}", self.x)
        }
        else {
            println!("The largest member is y: {}", self.y)
        }
    }
}

// Likewise we can also conditionally implement trait for a type that
// implements another trait
// This is how ToString trait is implemented by the std lib
// let s = 2.to_string();
trait Print {
    fn print(&self);
}

impl<T: Display> Print for T {
    fn print(&self) {
        println!("{self}")
    }
}
