// Advent of Code Day EIGHT
//
// Some imports not needed every time.
use a_o_c::*; //import custom lib.rs module
use std::io::Error;

#[derive(Debug, Clone)]
pub struct CodedData<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
}

#[derive(Debug, Clone)]
pub struct CodedData2<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
    // insert hashmap here
}

pub fn day8_challenge1(config: &Config) -> Result<i128, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let mut coded_data: Vec<CodedData> = Vec::new();
    for line in lines.iter() {
        // Split the input and output on the vertical line and create vectors for each.
        let half: Vec<&str> = line.split('|').collect();
        let input_half: Vec<&str> = half[0].split_whitespace().collect();
        let output_half: Vec<&str> = half[1].split_whitespace().collect();
        coded_data.push(CodedData {
            input: input_half,
            output: output_half,
        });
    }
    println!("{:?}", coded_data);
    let mut charcount: usize = 0;
    for one_line in coded_data.iter() {
        for character in one_line.output.iter() {
            // The first task is easy to just count the characters.
            match character.len() {
                2 => charcount += 1,
                3 => charcount += 1,
                4 => charcount += 1,
                7 => charcount += 1,
                _ => (),
            }
        }
    }
    println!("charcount {}", charcount);
    Ok(charcount as i128)
}

pub fn day8_challenge2(config: &Config) -> Result<i128, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let mut coded_data: Vec<CodedData> = Vec::new();
    for line in lines.iter() {
        // Split the input and output on the vertical line and create vectors for each.
        let half: Vec<&str> = line.split('|').collect();
        let input_half: Vec<&str> = half[0].split_whitespace().collect();
        let output_half: Vec<&str> = half[1].split_whitespace().collect();
        coded_data.push(CodedData {
            input: input_half,
            output: output_half,
        });
    }
    println!("{:?}", coded_data);
    let mut charcount: usize = 0;
    for one_line in coded_data.iter() {
        for character in one_line.output.iter() {
            // NOTE: TO BE MODIFIED IN NEXT REVISION. THIS IS JUST A COPY OF ABOVE.
            match character.len() {
                2 => charcount += 1,
                3 => charcount += 1,
                4 => charcount += 1,
                7 => charcount += 1,
                _ => (),
            }
        }
    }
    println!("charcount {}", charcount);
    // Ok(charcount as i128)
    Ok(0)
}
