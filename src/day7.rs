// Advent of Code Day SEVEN
//
// Some imports not needed every time.
use a_o_c::*; //import custom lib.rs module
use std::io::Error;

pub fn day7_challenge1(config: &Config) -> Result<i128, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let mut crab_horizontals: Vec<usize> =
        parse_string_line_into_integers_usize(lines.get(0).unwrap().to_string(), ',');
    // println!("crab horizontals {:?}", crab_horizontals);
    // println!("sum {}", crab_horizontals.iter().sum::<usize>());
    // calculate median.  This is the least distance for total travel to everyone.
    crab_horizontals.sort_unstable();
    let mid = crab_horizontals.len() / 2;
    let median = crab_horizontals[mid];
    let mut total_fuel: usize = 0;
    for crab in crab_horizontals.iter() {
        let fuel: usize = (*crab as i32 - median as i32).abs().try_into().unwrap();
        total_fuel += fuel;
    }
    println!("Challenge 1 median {}", median);
    println!("Challenge 1 total_fuel {}", total_fuel);
    Ok(total_fuel as i128)
}

pub fn day7_challenge2(config: &Config) -> Result<i128, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let mut crab_horizontals: Vec<usize> =
        parse_string_line_into_integers_usize(lines.get(0).unwrap().to_string(), ',');
    // println!("crab horizontals {:?}", crab_horizontals);
    // println!("sum {}", crab_horizontals.iter().sum::<usize>());
    // Closure to calculate fuel burn, which is 1/2 * x(x+1)
    let fuel_burn = |x: usize| (x * (x + 1)) / 2; //where x is distance moved.
                                                  // calculate median.  This is the least distance for total travel to everyone.
                                                  // for challenge 2, this will be the starting point of a numberical computation.
    crab_horizontals.sort_unstable();
    let mid = crab_horizontals.len() / 2;
    let median = crab_horizontals[mid];

    let mut least_total_fuel: Vec<usize> = vec![usize::MAX; 2]; // element 0 value, element 1 total fuel amount
                                                                // exhaustive O(n^) search. This is a start. I'd rather work outward from a median.
    for search_point in 0..crab_horizontals[crab_horizontals.len() - 1] {
        let mut total_fuel: usize = 0;
        for crab in crab_horizontals.iter() {
            let search_distance: usize = (*crab as i32 - search_point as i32)
                .abs()
                .try_into()
                .unwrap();
            let fuel: usize = fuel_burn(search_distance);
            total_fuel += fuel;
        }
        if total_fuel < least_total_fuel[1] {
            least_total_fuel[0] = search_point;
            least_total_fuel[1] = total_fuel;
        }
        // println!("search point: {}, total fuel: {}", search_point, total_fuel);
    }
    println!(
        "Challenge 2 median: {}, best point: {},  total fuel: {}\n",
        median, least_total_fuel[0], least_total_fuel[1]
    );
    // println!("median {}", median);
    // println!("total_fuel {}", total_fuel);
    Ok(least_total_fuel[1] as i128)
}
