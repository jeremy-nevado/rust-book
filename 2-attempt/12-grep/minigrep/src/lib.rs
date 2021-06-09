use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
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
        let args: &[String] = &["test".to_string(), "the".to_string(), "poem.txt".to_string()];
        match Config::new(args) {
            Ok(_) => return Ok(()),
            Err(s) => return Err(s.to_string()),
        }
    }
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
