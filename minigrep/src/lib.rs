use std::env;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;

    for line in search(&config.query, &content, !config.ignore_case) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = if case_sensitive {
        query.to_string()
    } else {
        query.to_lowercase()
    };
    println!("Case sensitive: {case_sensitive}");
    for line in content.lines() {
        if (case_sensitive && line.contains(&query))
            || (!case_sensitive && line.to_lowercase().contains(&query))
        {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_case_sensitive() {
        let content = "lili\nlala\ntest\nLILI";
        assert_eq!(vec!["lili"], search("li", content, true));
        assert_eq!(vec!["LILI"], search("LI", content, true));
        assert!(search("Li", content, true).is_empty());
    }

    #[test]
    fn simple_case_insensitive() {
        let content = "lili\nlala\ntest\nLIli";
        assert_eq!(vec!["lili", "LIli"], search("li", content, false));
        assert_eq!(vec!["lili", "LIli"], search("Li", content, false));
        assert_eq!(vec!["lili", "LIli"], search("LI", content, false));
    }
}
