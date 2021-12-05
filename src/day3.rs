// Advent of Code Day 3
use a_o_c::*;  //import custom lib.rs module
// use std::fs::File;
// use std::error::Error;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
// use std::process;

#[derive(Debug, Copy, Clone)]
pub struct Propulsion {
    gamma: i32,
    epsilon: i32,
}
impl Propulsion {
    pub fn new() -> Result<Propulsion, &'static str>{
        let gamma = 0;
        let epsilon = 0;
        Ok(Propulsion{gamma,epsilon})
    }
}

fn count_bits<'a>(config: &Config, propulsion: &'a mut /*'a */Propulsion) -> Result<&'a Propulsion, std::io::Error> {
    let file_buffer=read_prep_puzzle_file(&config);
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut gamma_digit_position: Vec<i32> = vec![0;12];
    let mut epsilon_digit_position: Vec<i32> = vec![0;12];
    for line in file_buffer.unwrap().lines() {
        let mut digit_position_index: usize =0;
        for digit in line.unwrap_or_else(|_|"".to_string()).chars() {
            match digit {
                '0' => gamma_digit_position[digit_position_index]+=1,
                '1' =>  epsilon_digit_position[digit_position_index]+=1,
                _ => println!("invalid input"),
            }
            digit_position_index += 1;
        }
    }
    println!("gamma {:?}", gamma_digit_position);
    println!("epsilon {:?}", epsilon_digit_position);
    let rev_gamma_digit_position = gamma_digit_position.iter().rev();
    let mut rev_epsilon_digit_position = epsilon_digit_position.iter().rev();
    let mut digit_place = 0;
    for current_gamma_digit in rev_gamma_digit_position { 
        if current_gamma_digit > rev_epsilon_digit_position.next().unwrap() {
gamma += 2_i32.pow(digit_place);
        }
        else {
epsilon += 2_i32.pow(digit_place);
        }
        digit_place +=1;
    }
    println!("digit places {}",digit_place);
    propulsion.gamma = gamma;
    propulsion.epsilon = epsilon;
    Ok(propulsion)
}


pub fn day3_challenge1(config: &Config) -> Result<i32, Error> {
    let mut ship_propulsion = Propulsion::new().unwrap();
    count_bits(config, &mut ship_propulsion);
    println!("propulsion {:?}",ship_propulsion);
    Ok(ship_propulsion.gamma*ship_propulsion.epsilon)
}

pub fn day3_challenge2(config: &Config) -> Result<i32, Error> {
    Ok(0)
}

