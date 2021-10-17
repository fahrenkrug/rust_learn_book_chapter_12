mod shoe;

use std::fs;
use std::error::Error;
use std::env;
use std::fs::read_to_string;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Iter;

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

    #[test]
    fn iterator_demonstration() {
        let vec = vec![1,2,3];
        // mut is necessary because next() changes the internal state.
        let mut iter = vec.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let vec = vec![1,2,3];
        let total: u32 = vec.iter().sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        // Just calling .map without passing this value into a variable would have triggered a warning
        // because iterators are lazy and only compute when they are really needed to do so.
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let v1_iterated: Vec<u32> = v1_iter.map(|x| x + 1).collect();
        assert_eq!(v1_iterated, vec![2,3,4]);
        let total: u32 = v1_iterated.iter().sum();
        assert_eq!(total, 9);
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get any query string")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name")
        };
        let should_use_case_insensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            should_use_case_insensitive
        })
    }
}