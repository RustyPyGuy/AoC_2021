// Day 1 Advent of Code
use a_o_c::*;
// use std::error::Error;
use std::fs::File;
// use std::error::Error;
// use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::io::Error;
use std::process;


pub fn day1_challege1(config: &Config) -> Result<i32, Error> {
    let f = File::open(config.filename.clone()).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    let contents = read_by_lines(f).expect("failed");
    let mut sum: i32 = 0;
    let mut compare = i32::MAX;
    // iterate over the vector
    for item in contents.into_iter() {
        if item > compare {
            sum +=1;
        }
        compare = item;
    }
    Ok(sum)
}

pub fn day1_challenge2(config: &Config) -> Result<i32, Error> {
    let f = File::open(config.filename.clone()).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    let contents = read_by_lines(f).expect("failed");
    let mut sum: i32 = 0;
    let mut sum_compare = i32::MAX;
    let mut window: Vec<i32> = vec![i32::MAX/3;3];
    
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
