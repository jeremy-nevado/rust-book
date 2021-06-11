use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_fail() -> Result<(), String> {
        let args: &[String] = &[];
        match Config::new(args) {
            Err(_) => return Ok(()),
            Ok(_) => return Err(String::from("Error was not thrown. Expected Error.")),
        }
    }

    #[test]
    fn new_pass() -> Result<(), String> {
        let args: &[String] = &[
            "test".to_string(),
            "the".to_string(),
            "poem.txt".to_string(),
        ];
        match Config::new(args) {
            Ok(_) => return Ok(()),
            Err(s) => return Err(s.to_string()),
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // Alternate implementation for case_sensitive
        // let case_sensitive = match args.get(3) {
        //      Some(_) => true,
        //      None => false,
        // }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}
