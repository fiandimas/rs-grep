use std::{env, process};

use rsgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = rsgrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}