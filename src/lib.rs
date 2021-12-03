// Advent of code
use std::fs::File;
// use std::error::Error;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::process;

pub struct Config {
    pub challenge: u32,
    pub filename: String,
    // pub case_sensitive: bool,
}

impl Config {
    pub fn parse_config(args:&[String]) -> Result<Config, Error> {
        let challenge: u32 = args[1].clone().parse().unwrap();
        let filename = args[2].clone();
        // println!("arguments parsed {}, {}", challenge, filename);
        return Ok(Config{challenge, filename });
    }
}

pub fn run1(config: Config) -> Result<u32, Error> {
    let f = File::open(config.filename).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    let contents = read_by_lines(f).expect("failed");
    let mut sum: u32 = 0;
    let mut compare = u32::MAX;
    // iterate over the vector
    for item in contents.into_iter() {
        if item > compare {
            sum +=1;
        }
        compare = item;
    }
    Ok(sum)
}

pub fn run2(config: Config) -> Result<u32, Error> {
    let f = File::open(config.filename).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    let contents = read_by_lines(f).expect("failed");
    let mut sum: u32 = 0;
    let mut sum_compare = u32::MAX;
    let mut window: Vec<u32> = vec![u32::MAX/3;3];
    
    // iterate over the vector
    // This algorithm assigns values to a new sliding window vector. 
    // A more memory and processor efficient algorithm would work on the original vector.
    for item in contents.into_iter() {
        window.remove(0);
        window.push(item);
        let sum_window = window[0] + window[1] + window[2];
            if (sum_window) > sum_compare {
            sum +=1;
        }
        sum_compare = sum_window;
    }
    Ok(sum)
}
// fn read<R: Read>(io: R) -> Result<Vec<u32>, Box<dyn Error>> {
//     let br = BufReader::new(io);
//     br.lines()
//         .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
//         .collect()
// }
fn read_by_lines<R: Read>(io: R) -> Result<Vec<u32>, std::io::Error> {
    let br = BufReader::new(io);
    let mut v = Vec::<u32>::with_capacity(2048);
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}
