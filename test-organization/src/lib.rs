use std::ops::Add;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn private_add<T>(left: T, right: T) -> T
where
    T: Add<Output = T>
{
    left + right
}

// Here cfg is configuration and says the following should be only included
// in certain configuration i.e. the test configuration in this case
// hence, only runs with cargo test
// This is unit tests for this module and hence needs this attribute to seperate
// the module code and test code
#[cfg(test)]
mod tests {
    // Since tests is just another module, it has access to private members of
    // it parent module and hence we can also test private functions
    use super::*;

    #[test]
    fn test_private_add() {
        let result = private_add(2, 2);
        assert_eq!(result, 4, "Add operation is incorrect!");
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
