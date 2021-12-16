// Advent of Code Day EIGHT
//
// Some imports not needed every time.
use a_o_c::*; //import custom lib.rs module
use std::io::Error;
// use std::collections::HashMap;
use bimap::BiHashMap;

#[derive(Debug, Clone)]
pub struct CodedData<'a> {
    input: Vec<&'a str>,
    output: Vec<&'a str>,
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
    // println!("{:?}", coded_data);
    let mut charcount: usize = 0;
    for one_line in coded_data.iter() {
        for segment in one_line.output.iter() {
            // The first task is easy to just count the segments.
            match segment.len() {
                2 => charcount += 1,
                3 => charcount += 1,
                4 => charcount += 1,
                7 => charcount += 1,
                _ => (),
            }
        }
    }
    // println!("charcount {}", charcount);
    Ok(charcount as i128)
}

#[derive(Debug, Clone)]
pub struct CodedData2<'a> {
    // input: Vec<&'a str>,
    input: Vec<String>,
    output: Vec<String>,
    translation: BiHashMap<&'a str, Option<usize>>,
    translation2: BiHashMap<&'a str, &'a str>,
}
pub fn day8_challenge2(config: &Config) -> Result<i128, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let mut coded_data: Vec<CodedData2> = Vec::new();
    for line in lines.iter() {
        // Split the input and output on the vertical line and create vectors for each.
        let half: Vec<&str> = line.split('|').collect();
        // let input_half: Vec<&str> = half[0].split_whitespace().collect();
        let input_half: String = String::from(half[0]); //NOTE: This may be the way.
                                                        // create a vector of strings so that it can be manipulated.
        let input_half2: Vec<String> = input_half.split(" ").map(|x| x.to_owned()).collect();
        // .split_whitespace().collect::<Vec<String>>();
        // let output_half: Vec<&str> = half[1].split_whitespace().collect();

        let output_half: String = String::from(half[1]); //NOTE: This may be the way.
                                                         // create a vector of strings so that it can be manipulated.
        let output_half2: Vec<String> = output_half.split(" ").map(|x| x.to_owned()).collect();

        fn sort_character_chunks3<'a>(input: &'a Vec<String>) -> Vec<String> {
            let mut output_vec: Vec<String> = Vec::new();
            for mut chunk in input.iter() {
                let mut tempsort: Vec<char> = chunk.chars().collect();
                tempsort.sort_unstable();
                let tempsort2: String = tempsort.into_iter().collect();
                output_vec.push(tempsort2);
            }
            output_vec
        }

        coded_data.push(CodedData2 {
            input: sort_character_chunks3(&input_half2),
            output: sort_character_chunks3(&output_half2),
            translation: BiHashMap::with_capacity(10),
            translation2: BiHashMap::with_capacity(10),
        });
    }

    let mut total_sum: usize = 0;
    println!("{:?}", coded_data);
    // let mut charcount: usize = 0;
    for one_line in coded_data.iter() {
        // bidirectional hash map.
        let mut temp_translation_map2: BiHashMap<&str, &str> = BiHashMap::with_capacity(10);
        let mut temp_translation_map3: BiHashMap<&str, &str> = BiHashMap::with_capacity(10);
        let mut decode_string_temp: String = String::with_capacity(10);

        // First pass to decode - based on length.
        for segment in one_line.input.iter() {
            match segment.len() {
                2 => {
                    temp_translation_map2.insert(segment, "1"); // ONE
                    temp_translation_map3.insert(segment, "1");
                }
                3 => {
                    temp_translation_map2.insert(segment, "7"); // SEVEN
                    temp_translation_map3.insert(segment, "7");
                }
                4 => {
                    temp_translation_map2.insert(segment, "4"); // FOUR
                    temp_translation_map3.insert(segment, "4");
                }
                7 => {
                    temp_translation_map2.insert(segment, "8"); // EIGHT
                    temp_translation_map3.insert(segment, "8");
                }
                _ => {} // no action for other digits.
            }
        }
        // closure to facilitate a comparison of character counts of a string
        let count_compare2 = |A: &str, B: &str| {
            let mut count: usize = 0;
            let B_ref = &temp_translation_map3.get_by_right(B);
            for c in A.chars() {
                for d in B_ref.unwrap().chars() {
                    if c == d {
                        count += 1;
                    }
                }
            }
            count
        };
        for segment in one_line.input.iter() {
            if segment.len() == 5 {
                // TWO, THREE, FIVE have 5 segments displayed
                if count_compare2(segment, "4") == 2 {
                    // TWO uniquely shares only 2 segments with FOUR of characters with 5 total segments
                    temp_translation_map2.insert(segment, "2");
                    // println!("2 matched 2awesome!");
                } else if count_compare2(segment, "7") == 3 {
                    // THREE uniquely shares all three segments of SEVEN of characters with 5 total segments.
                    temp_translation_map2.insert(segment, "3");
                    // println!("3 matched 2awesome!");
                } else if count_compare2(segment, "4") == 3 && count_compare2(segment, "1") == 1 {
                    // FIVE uniquely shares 3 segments of FOUR AND only 1 segment of ONE with 5 total segments.
                    temp_translation_map2.insert(segment, "5");
                    // println!("5 matched 2awesome!");
                }
            }
            if segment.len() == 6 {
                // SIX, NINE, ZERO have 6 segments displayed
                if count_compare2(segment, "1") == 1 {
                    // SIX uniquely only shares 1 segment of ONE with 6 total segments.
                    temp_translation_map2.insert(segment, "6");
                    // println!("6 matched 2awesome!");
                } else if count_compare2(segment, "7") == 3 && count_compare2(segment, "4") == 4 {
                    // NINE uniquely shares all 3 segment of SEVEN and all 4 of FOUR with 6 total segments.
                    temp_translation_map2.insert(segment, "9");
                    // println!("9 matched 2wesome!");
                } else if count_compare2(segment, "4") == 3 && count_compare2(segment, "7") == 3 {
                    // ZERO uniquely shares only 3 segments of FOUR and all 3 segments of SEVEN with 6 total segments.
                    temp_translation_map2.insert(segment, "0");
                    // println!("0 matched 2wesome!");
                }
            }
        }
        // println!("hashmap {:?}", temp_translation_map);
        println!("bihashmap {:?}", temp_translation_map2);
        for out_segment in one_line.output.iter() {
            // println!("out_segment {:?}", out_segment);

            println!(
                "out_decoded {:?}",
                temp_translation_map2.get_by_left(out_segment.as_str()) /*.unwrap_or_else(||&"")*/
            );
            decode_string_temp.push_str(
                temp_translation_map2
                    .get_by_left(out_segment.as_str())
                    .unwrap_or_else(|| &""),
            );
            // decode_string_temp.push_str(temp_translation_map2.get_by_left(out_segment.as_str()).unwrap_or_else(||&""));
        }
        println!("output {}", decode_string_temp);
        total_sum += decode_string_temp.parse::<usize>().unwrap_or(0);
    }
    // Ok(charcount as i128)
    Ok(total_sum as i128)
}
