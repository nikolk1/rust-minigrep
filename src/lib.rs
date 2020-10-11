use std::error::Error;
use std::{fs, io, env};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
        let case_sensitive = true;

        if !path_exists(&filename) {
            return Err("File doesn't exist");
        }
        Ok(Config {
            command,
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents, config.case_sensitive) {
        highlight(&line, &config.query)?;
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

fn highlight(line: &str, word: &str) -> io::Result<()> {
    //TODO: make the color enviroment variable
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let color: Color = match env::var("COLOR") {
        Ok(val) => Color::from_str(&val).unwrap_or(Color::Red),
        Err(e) => Color::Red
    };

    let line_str = String::from(line);
    let start = line.find(&word).expect("Couldn't find word in line");
    let end = start + word.len();

    stdout.reset()?;
    write!(&mut stdout, "{}", &line_str[0..start]);

    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    write!(&mut stdout, "{}", &line_str[start..end])?;

    stdout.reset()?;
    writeln!(&mut stdout, "{}", &line_str[end..]);
    Ok(())
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
