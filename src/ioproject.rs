// From: https://github.com/rust-lang/book/blob/main/listings/ch12-an-io-project/listing-12-04/src/main.rs

use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[test]
fn test_main() {
    //let args: Vec<String> = env::args().collect();
    let args = vec!["arg-0", "println", "/Users/adarshrp/Projects/squareone/src/ioproject.rs"];

    let config = Config::new(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("--- Running grep {} {} --", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for matched_line in contents.split('\n').filter(|e| e.contains(&config.query)) {
        println!("  : {}", matched_line);
    }

    Ok(())
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: Vec<&str>) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
