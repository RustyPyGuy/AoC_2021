// Advent of Code 2021!
// Structs and methods used for each or most days are included in this lib.rs file
// This includes things like opening files and reading input.
use std::fs::File;
// use std::error::Error;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::process;


pub struct Config {
    pub challenge: i32,
    pub filename: String,
    // pub case_sensitive: bool,
}

impl Config {
    pub fn parse_config(args:&[String]) -> Result<Config, Error> {
        let challenge: i32 = args[1].clone().parse().unwrap();
        let filename = args[2].clone();
        // println!("arguments parsed {}, {}", challenge, filename);
        Ok(Config{challenge, filename })
    }
}

pub fn read_by_lines<R: Read>(io: R) -> Result<Vec<i32>, std::io::Error> {
    let br = BufReader::new(io);
    let mut v = Vec::<i32>::with_capacity(2048);
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}
pub fn read_prep_puzzle_file/*<R: Read, T>*/(config: &Config) -> Result<std::io::BufReader<File>, Error> {
    let f = File::open(config.filename.clone()).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    let br = BufReader::new(f);
    Ok(br)
}
