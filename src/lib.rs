use std::fs;
use std::error::Error;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents))
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Tutorial implementation
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // My own implementation
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename).expect("Something went wrong when reading the file");
    let results = if config.should_use_case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    results.iter().for_each(|line| println!("{}", line));

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub should_use_case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments passed to application.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let should_use_case_insensitive = match env::var("CASE_INSENSITIVE") {
            Err(e) => false,
            Ok(string) => string != "0"
        };
        Ok(Config { query, filename, should_use_case_insensitive })
    }
}