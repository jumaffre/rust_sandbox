use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() != 3 {
            // panic!("Error: usage {} <query> <file>", args[0]);
            return Err("function takes 3 arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    println!("File content: {}", content);

    Ok(())
}
