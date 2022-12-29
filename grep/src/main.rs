use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "enter the pattern")]
#[command(propagate_version = true)]
struct Cli {
    #[command()]
    pattern: String,
}

fn process_line<T: BufRead>(reader: T, regex: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match regex.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let pattern = Cli::parse().pattern;
    let regex = Regex::new(pattern.as_str()).unwrap();

    println!("pattern is {:?}", pattern);

    let file = match File::open("./src/text.txt") {
        Ok(file) => file,
        Err(e) => return println!("{:?}", e),
    };

    let reader = BufReader::new(file);
    process_line(reader, regex);
}
