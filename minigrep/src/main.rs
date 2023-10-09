use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!(
            "Error parsing arguments: usage migrep <query> <file_path>: {}",
            err
        );
        process::exit(1);
    });

    // Note: unwrap_or_else() would work too but because returns () on success, we don't need to unwrap the result
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
