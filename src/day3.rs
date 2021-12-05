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
    co2: i32,
    oxygen: i32,
}
impl Propulsion {
    pub fn new() -> Result<Propulsion, &'static str>{
        let gamma = 0;
        let epsilon = 0;
        let co2 = 0;
        let oxygen = 0;
        Ok(Propulsion{gamma,epsilon,co2,oxygen})
    }
}

fn count_bits_puzzle1<'a>(config: &Config, propulsion: &'a mut /*'a */Propulsion) -> Result<&'a Propulsion, std::io::Error> {
    let file_buffer=read_prep_puzzle_file(&config);
    let mut gamma = 0;
    let mut epsilon = 0;
    let mut gamma_digit_position: Vec<i32> = vec![0;12];
    let mut epsilon_digit_position: Vec<i32> = vec![0;12];
    // Looping through all of the lines and characters and storing counts of each
    // character position in a vector to be compared later.
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
    // Comparing each vector of characters in reverse order for the most and least
    // Immediately computing the decimal equivalent.
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


fn count_bits_puzzle2<'a>(config: &Config, propulsion: &'a mut /*'a */Propulsion) -> Result<&'a Propulsion, std::io::Error> {
    let file_buffer=read_prep_puzzle_file(&config);
    // Taking another approach completely now:
    // For the oxygen measurement, Looping through and creating 2 lists.
    // leading with 1 and leading with 0.  Discarding the smaller of the two
    // and continuing to remove leading digits and loop through.
    // This can probably be done recursively.
    //
    let mut line_vector:  Vec<String> = Vec::new();

    let mut leading_ones_list: Vec<String> = Vec::new();
    for line in file_buffer.unwrap().lines() {
    line_vector.push(line.unwrap()); 
    }
    let test_out = recurse_leading_digit_select(line_vector.clone(),"oxygen",None);
    let test_out_dec = string_binary_to_integer(test_out[0].clone());
    propulsion.oxygen = test_out_dec;
    println!("test out {:?}, {}",&test_out[0].clone(),test_out_dec);
    let test_out = recurse_leading_digit_select(line_vector,"co2",None);
    let test_out_dec = string_binary_to_integer(test_out[0].clone());
    propulsion.co2 = test_out_dec;
    println!("test out {:?}, {}",&test_out[0].clone(),test_out_dec);

    // A simple fuction to convert the binary String representation to an i32
    fn string_binary_to_integer(mut string_binary: String) -> i32 {
        //revserse string and create iterator
        // let rev_string_binary: String = string_binary.iter().rev();
        let mut digit_place: u32 = 0;
        let mut decimal_out: i32 = 0;
        for _ in 0..string_binary.len() { 
           let digit: Option<char> = string_binary.pop();
            if digit == Some('1') {
            decimal_out += 2_i32.pow(digit_place);
                }
            digit_place +=1;
        }
        decimal_out
    }
 
    
    // Recursive function to divide the data into two different vectors starting
    // with zero or one at the digit place of interest, and then returning the one set that meets
    // the criteria.  Recurses on the output of this to further divide and conquer until only one
    // string remains.  The other design option would have been nested loops.
    fn recurse_leading_digit_select(input_vector: Vec<String>, profile: &str, digit_index_option: Option<i32>) ->  Vec<String> {
            // BASE RECURSION CASE. A single element remains. Do nothing and return.
            if input_vector.len() == 1 {
                return input_vector;
            }
        let digit_index: i32 = digit_index_option.unwrap_or(0); // digit under inspection.
        let mut output_vector:  Vec<String>;
        let mut leading_ones_list: Vec<String> = Vec::new();
        let mut leading_zeros_list: Vec<String> = Vec::new();
        let mut selected_list: Vec<String>;

        for line in input_vector.iter() {
           let digit: Option<char> = Some(line.clone().remove(digit_index.try_into().unwrap()));
            match digit {
                Some('0') => leading_zeros_list.push(line.to_string()),
                Some('1') =>  leading_ones_list.push(line.to_string()),
                Some(_)=> println!("invalid input character. Not 1 or 0"),
                None => ()/*println!("invalid evaluation input")*/,
            }
        }
        if profile == "oxygen" {
            if leading_ones_list.len() >= leading_zeros_list.len() {
            selected_list = leading_ones_list;
            }
            else{
            selected_list = leading_zeros_list;
            }
        }
        else if profile == "co2" {
            if leading_zeros_list.len() <= leading_ones_list.len() {
            selected_list = leading_zeros_list;
            }
            else{
            selected_list = leading_ones_list;
            }
            
        }
        else {
            panic!("invalid profile passed."); //panic is not perfect, but whatever for now.
        }
        println!("length of selected list: {:?}",selected_list.len());
        // Recurse and advance the digit index.
        output_vector = recurse_leading_digit_select(selected_list,profile,Some(digit_index+1));
        output_vector
    } //end fn definition
    Ok(propulsion)
}

// The launch functions
pub fn day3_challenge1(config: &Config) -> Result<i32, Error> {
    let mut ship_propulsion = Propulsion::new().unwrap();
    count_bits_puzzle1(config, &mut ship_propulsion);
    println!("propulsion {:?}",ship_propulsion);
    Ok(ship_propulsion.gamma*ship_propulsion.epsilon)
}

pub fn day3_challenge2(config: &Config) -> Result<i32, Error> {
    let mut ship_propulsion = Propulsion::new().unwrap();
    count_bits_puzzle2(config,&mut ship_propulsion);
    Ok(ship_propulsion.oxygen*ship_propulsion.co2)
}

