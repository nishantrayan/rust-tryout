use std::env;
use std::fs;
use std::error::Error;
use std::process;

struct Config {
    query: String,
    file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file = args[2].clone();

        Ok(Config { query, file })
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file);
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);   
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    println!("Contents = {contents}");
    Ok(())
}
