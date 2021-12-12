// Advent of Code 2021!
// Structs and methods used for each or most days are included in this lib.rs file
// This includes things like opening files and reading input.
use std::fs::File;
use std::fs;
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

pub fn read_prep_puzzle_file_contents_to_string/*<R: Read, T>*/(config: &Config) -> String {
    // let f = File::open(config.filename.clone()).unwrap_or_else(|err| {
    //     println!("Error opening file: {}", err);
    //     process::exit(1);
    // });
    let file_contents = fs::read_to_string(config.filename.clone()).unwrap();
    file_contents
} 

pub fn iterate_by_lines_from_string<'a>(input: &'a String) -> Vec<&'a str> {
    let lines: Vec<&'a str> = input.lines().filter(|l| l != &"").collect();
    // let output = lines.iter();
    // output
    lines

}
// pub fn iterate_by_lines_from_string(input: String, output: Vec<&'static mut str>) -> Vec<&'static mut str> {
//     let mut lines: Vec<&'static mut str> = input.lines().filter(|l| l != &"").collect();
//     // let output = lines.iter();
//     // output
//     output = lines;
//     output
// }
pub fn parse_string_line_into_integers(input:String, delimiter: char) -> Vec<u32>{
    // let lines: Vec<&str> = input.lines().filter(|l| l != &"").collect();
    let numbers: Vec<u32> = input
        .split(&delimiter.to_string())
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    numbers
}

// fn get_numbers_and_boards(file: &str) -> (Vec<u32>, Vec<BingoBoard>) {
//     let file_contents = fs::read_to_string(file).unwrap();
//     let input: Vec<&str> = file_contents.lines().filter(|l| l != &"").collect();
//     let numbers: Vec<usize> = input[0]
//         .split(",")
//         .map(|s| s.parse::<usize>().unwrap())
//         .collect();
// 
//     let board_input: Vec<BingoBoard> = input[1..].chunks(5).map(|c| BingoBoard::build(c)).collect();
// 
//     (numbers, board_input)
// }
