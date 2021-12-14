// Advent of Code Day TEMPLATE
//
// Some imports not needed every time.
use a_o_c::*; //import custom lib.rs module
use std::io::Error;
use std::collections::VecDeque;
const GROWTH_CYCLES1: usize = 80; // growth cycles for challenge 1
const GROWTH_CYCLES2: usize = 256; // growth cycles for challenge 2

pub fn day6_challenge1(config: &Config) -> Result<i32, Error> {
    #[allow(unused_assignments)]
    let mut lanternfish: Vec<usize> = Vec::new();
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    lanternfish = parse_string_line_into_integers_usize(lines.get(0).unwrap().to_string(), ',');
    // println!("input, {:?}",lanternfish);
    let mut newfish: Vec<usize> = Vec::new();
    for _day in 0..GROWTH_CYCLES1 {
        for fish in lanternfish.iter_mut() {
            match fish{
            0 =>  {*fish =6; newfish.push(8);},
            1 => {*fish = 0}, 
            2 => {*fish = 1}, 
            3 => {*fish = 2}, 
            4 => {*fish = 3}, 
            5 => {*fish = 4}, 
            6 => {*fish = 5}, 
            7 => {*fish = 6}, 
            8 => {*fish = 7}, 
            _ => {panic!("invalid fish")},
            }
        }
        lanternfish.append(&mut newfish);
    }
    Ok(lanternfish.len() as i32)

}

pub fn day6_challenge2(config: &Config) -> Result<usize, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let lanternfish: Vec<usize> = parse_string_line_into_integers_usize(lines.get(0).unwrap().to_string(), ',');
    // Vectors to hold counts of lanternfish. The index is the day of aging age of the fish.
    // Using a VecDeque because of it's ability to rotate end to end.
    #[allow(unused_assignments)]
    let mut lanternfish_population: VecDeque<usize> = VecDeque::with_capacity(9);
    lanternfish_population = VecDeque::from([0;9]);
    for fish in lanternfish.iter() {
        // There may be a more space efficient way to do this, but this is probably execution
        // efficient.
        match fish{
        0 => {lanternfish_population[0] += 1;},
        1 => {lanternfish_population[1] += 1;}, 
        2 => {lanternfish_population[2] += 1;}, 
        3 => {lanternfish_population[3] += 1;}, 
        4 => {lanternfish_population[4] += 1;}, 
        5 => {lanternfish_population[5] += 1;}, 
        6 => {lanternfish_population[6] += 1;}, 
        7 => {lanternfish_population[7] += 1;}, 
        8 => {lanternfish_population[8] += 1;}, 
        _ => {panic!("invalid fish")},
        }
    }
    #[allow(unused_assignments)]
    let mut lanternfish_population_to_6: VecDeque<usize> = VecDeque::with_capacity(9);
    lanternfish_population_to_6 = lanternfish_population;
    #[allow(unused_assignments)]
    let mut lanternfish_population_7_8: VecDeque<usize> = VecDeque::with_capacity(2);
    // splitting the 0-6 range and the 7-8 range to account for different growth start points.
    lanternfish_population_7_8 = lanternfish_population_to_6.split_off(7);
    // println!("two vectors 6, 7-8, {:?}  {:?}",lanternfish_population_to_6, lanternfish_population_7_8);

    for _day in 0..GROWTH_CYCLES2 {
        // note the number of fish that will grow
        let lanternfish_growth: usize = lanternfish_population_to_6[0];
        // Standard aging (day decrease)
        lanternfish_population_to_6.rotate_left(1);
        // fish from 6 to 7 and 7 to 8
        lanternfish_population_to_6[6] += lanternfish_population_7_8.pop_front().unwrap();
        // new fish at 8
        lanternfish_population_7_8.push_back(lanternfish_growth);

    // println!("two vectors 6, 7-8, {:?}  {:?}",lanternfish_population_to_6, lanternfish_population_7_8);
    }
    lanternfish_population_to_6.append(&mut lanternfish_population_7_8);
    let total_lanternfish: usize = lanternfish_population_to_6.iter().sum();
    println!("lanternfish population post-growth, {:?}",lanternfish_population_to_6);
    println!("total lanternfish population post-growth, {}",total_lanternfish);
    Ok(total_lanternfish)
}
