/*
 * Advent of Code 2021 - RustyPiGuy
 *
 */
use a_o_c::*; //import lib module
mod day1;
use day1::*; // Day 1
mod day2;
use day2::*; // Day 2
mod day3;
use day3::*; // Day 3
mod day4;
use day4::*;
mod day5;
use day5::*;
mod day6;
use day6::*;
use std::env;
use std::process;

fn main() {
    println!("Advent of Code!");
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_config(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // NOTE: Need code for incorrect number of arguments
    let mut result: Vec<i128> = Vec::new();
    match config.challenge {
        1 => {
            result.push(day1_challege1(&config).unwrap().into());
            result.push(day1_challenge2(&config).unwrap().into());
        }
        2 => {
            result.push(day2_challenge1(&config).unwrap().into());
            result.push(day2_challenge2(&config).unwrap().into());
        }
        3 => {
            result.push(day3_challenge1(&config).unwrap().into());
            result.push(day3_challenge2(&config).unwrap().into());
        }
        4 => {
            result.push(day4_challenge1(&config).unwrap().into());
            result.push(day4_challenge2(&config).unwrap().into());
        }
        5 => {
            result.push(day5_challenge1(&config).unwrap().into());
            result.push(day5_challenge2(&config).unwrap().into());
        }
        6 => {
            result.push(day6_challenge1(&config).unwrap().into());
            result.push(day6_challenge2(&config).unwrap().try_into().unwrap());
        }
        _ => {
            println!("Invalid challenge input. Exiting");
            process::exit(1);
        }
    };
    println!(
        "The results for Day {} are:\n\
        Challenge 1 result {}\nChallenge 2 result {}",
        config.challenge, result[0], result[1]
    );
}
