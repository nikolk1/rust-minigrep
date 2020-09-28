use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        usage();
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }
}

fn check_args(args: &Vec<String>) -> bool {
    if args.len() != 3 {
        usage();
        return false;
    }
    true
}

fn usage() {
    println!("minigrep <pattern> <filname>")
}
