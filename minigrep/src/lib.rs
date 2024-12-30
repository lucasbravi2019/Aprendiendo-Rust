use std::{error::Error, fs, env};

const TRUE: &str = "true";
const IGNORE_CASE: &str = "IGNORE_CASE";
const TRUE_DIGIT: &str = "1";

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = if let Some(arg) = args.next() {
            arg.to_lowercase() == TRUE || arg == TRUE_DIGIT
        } else {
            env::var(IGNORE_CASE).is_ok()
        };

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();

    content.lines().filter(|line| line.to_lowercase().contains(&query_lower)).map(|line| line.trim()).collect()
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| line.contains(query)).map(|line| line.trim()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
        Rust: 
        safe, fast, productive.
        Pick three.
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content));
    }
}
