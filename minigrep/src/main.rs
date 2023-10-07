use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!(
            "Error parsing arguments: usage {} <query> <file_path>: {}",
            args[0], err
        );
        process::exit(1);
    });

    println!("Configuration: {:?}", config);
    println!("Query: {}", config.query);

    // Note: unwrap_or_else() would work too but because returns () on success, we don't need to unwrap the result
    if let Err(e) = minigrep::run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
