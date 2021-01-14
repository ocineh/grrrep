use std::{fs, error::Error};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("not enough arguments"); }
        Ok(Config { query: args[1].clone(), filename: args[2].clone() })
    }
}

// returns either OK or Err with a type with the Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // if fs::read_to_string returns an Err it is directly returned as the result of the run function thanks to the ?
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

// you have to indicate the shelf life to remove any cramping, the return and a slice of the content so they must have the same shelf life
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) { results.push(line); }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive\nPick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
    #[test]
    fn search_no_result() {
        let query = "nothing";
        let contents = "\nRust:\nsafe, fast, productive\nPick three.";
        let result: Vec<&str> = Vec::new();
        assert_eq!(result, search(query, contents));
    }
    #[test]
    fn search_two_result() {
        let query = "st";
        let contents = "\nRust:\nsafe, fast, productive\nPick three.";
        assert_eq!(vec!["Rust:", "safe, fast, productive"], search(query, contents));
    }
}