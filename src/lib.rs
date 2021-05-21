use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    search: String,
    file: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let search = match args.next() {
            Some(arg) => arg,
            None => return Err("Error boy"),
        };

        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Error boy"),
        };

        Ok(Config { search, file})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    for line in search(&config.search, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "safe";
        let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
}
