use std::{env, fs, process};
use std::error::Error;

struct Config {
    search: String,
    file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Error: not enough arguments");
        }
        let search= args[1].clone();
        let file = args[2].clone();

        Ok(Config { search, file})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;

    println!("{}", contents);
    Ok(())
}
