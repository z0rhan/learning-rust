pub fn search<'a>(
    query: &'a str,
    contents: &'a str
) -> impl Iterator<Item = &'a str>
{
    contents
        .lines()
        .filter(move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> impl Iterator<Item = &'a str>
{
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast and productive.
Pick three.
Duct tape";

        let mut iter = search(query, contents);
        assert_eq!("safe, fast and productive.",
            iter.next().expect("Iterator should not be empty"));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast and productive.
Pick three.
Trust me.";

        let mut iter = search_case_insensitive(query, contents);
        assert_eq!(
            "Rust:",
            iter.next().expect("Iterator should not be empty")
        );
        assert_eq!(
            "Trust me.",
            iter.next().expect("Iterator should not be empty")
        );
    }
}
