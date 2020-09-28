use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Config {
    pub command: String,
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enougth arguments");
        }
        let command = args[0].clone();
        let query = args[1].clone();
        let filename = args[2].clone();

        if !path_exists(&filename) {
            return Err("File doesn't exist");
        }
        Ok(Config {
            command,
            query,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, sensitive: bool) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if (!sensitive && line.to_lowercase().contains(&query.to_lowercase()))
            || (sensitive && line.contains(&query))
        {
            results.push(line);
        }
    }
    results
}

fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config() {}

    #[test]
    fn no_file() {}

    #[test]
    fn too_little_arguments() {}

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}
