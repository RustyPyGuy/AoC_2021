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
// fn read_file(filename: String) -> Result<(), std::io::Error> {
//     let vec = read2(fs::File::open(filename)?)?;
//     // use `vec` for whatever
//     Ok(())
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     #[test]
//     fn parse_args_err(){
//         // let input: Vec<String> = vec!("stuff1".to_string(),"stuff2".to_string(),"stuff3".to_string())/*.as_slice()*/;
//         let input: Vec<String> = vec!("stuff1".to_string())/*.as_slice()*/;
//         let config = Config::parse_config(&input);
//         assert_eq!(Config::parse_config(&input).err(), Some("Not enough arguments."))
//         // assert_eq!(config,Err("Not enough arguments."));
//     }

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive
// Duct Tape.
// pick three.
// Anthoner line of text.";
//         assert_eq!(
//             vec!["safe, fast, productive"],
//             search(query, contents)
//         );
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUst";
//         let contents = "\
// Rust:
// safe, fast, productive
// pick three.
// Anthoner line of text.
// Trust me.";
//         assert_eq!(
//             vec!["Rust:","Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// }
