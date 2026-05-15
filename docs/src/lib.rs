//! This is an example crate that shows how documentation works in Rust
// Comments with //! will belong to the item containing the comment
// For example, the first line is an documentation comment of this crate



// Below this is documentation comments
// Supports markdown formatting
// This will belong to the item that follows after
/// Adds one to given integer(i32)
///
/// # Examples
/// ```rust
///
/// let num: i32 = 1;
/// let num_plus_one = docs::add_one(num);
///
/// assert_eq!(num_plus_one, 2)
/// ```
pub fn add_one(num: i32) -> i32 {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
