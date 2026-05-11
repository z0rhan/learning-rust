use std::env;
use std::fs;
use std::process;
use std::error::Error;

use minigrep::{search, search_case_insensitive};

fn main() {
    let args_iter = env::args();

    let config = Config::build(args_iter).unwrap_or_else(|err| {
        eprintln!("Failed to parse arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

struct Config {
    query: String,
    filepath: String,
    ignore_case: bool
}

impl Config {
    // Here, earilier I did just &str for return type
    // Due to lifetime elision, it made the lifetime of &str tied to args
    // which does not give any error but its not true, LESSON LEARNED
    // fn build(mut args_iter: impl Iterator<Item = String>) -> Result<Self, &'static str> {
    fn build(mut args_iter: env::Args) -> Result<Self, &'static str> {
        args_iter.next();

        let query = match args_iter.next() {
            Some(val) => val,
            None => return Err("Not enough arguemetns")
        };

        let filepath = match args_iter.next() {
            Some(val) => val,
            None => return Err("Not enough arguemetns")
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query: query,
            filepath: filepath,
            ignore_case: ignore_case
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filepath)?;

    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}")
        }
    }
    else {
        for line in search(&config.query, &contents) {
            println!("{line}")
        }
    };

    Ok(())
}
