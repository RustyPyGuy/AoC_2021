// Advent of Code Day FIVE
//
use a_o_c::*; //import custom lib.rs module
use grid::*;
use std::io::Error;

const MAX_ROWS: usize = 1000;
const MAX_COLS: usize = 1000;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    c1: (usize, usize), // Tuple x,y
    c2: (usize, usize), // Tuple x,y
}
impl Coord {
    pub fn add(x1: usize, y1: usize, x2: usize, y2: usize) -> Coord {
        let c1_val = (x1, y1);
        let c2_val = (x2, y2);
        Coord {
            c1: c1_val,
            c2: c2_val,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mapper {
    mapped_points: Grid<usize>, // Contains a vector grid of integers that indicate whether a number is marked.
}
impl Mapper {
    pub fn new() -> Result<Mapper, &'static str> {
        let mapped_area = Grid::init(MAX_ROWS, MAX_COLS, 0);
        Ok(Mapper {
            mapped_points: mapped_area,
        })
    }
    pub fn draw_on_map(&mut self, input: &Coord) {
        if input.c1.0 == input.c2.0 {
            //Vertical line. Same X value
            let x = input.c1.0;
            if input.c1.1 > input.c2.1 {
                for y in input.c2.1..=input.c1.1 {
                    let mut temp_val: usize = self.mapped_points[x][y];
                    temp_val += 1;
                    self.mapped_points[x][y] = temp_val;
                    // println!("mapped value vertical {}",self.mapped_points[x][y]);
                }
            } else {
                for y in input.c1.1..=input.c2.1 {
                    let mut temp_val: usize = self.mapped_points[x][y];
                    temp_val += 1;
                    self.mapped_points[x][y] = temp_val;
                    // println!("mapped value vertical {}",self.mapped_points[x][y]);
                }
            }
        }
        if input.c1.1 == input.c2.1 {
            //Horizontal line. Same Y value
            let y = input.c1.1;
            if input.c1.0 > input.c2.0 {
                for x in input.c2.0..=input.c1.0 {
                    let mut temp_val: usize = self.mapped_points[x][y];
                    temp_val += 1;
                    self.mapped_points[x][y] = temp_val;
                    // println!("mapped value horizontal {}",self.mapped_points[x][y]);
                }
            } else {
                for x in input.c1.0..=input.c2.0 {
                    let mut temp_val: usize = self.mapped_points[x][y];
                    temp_val += 1;
                    self.mapped_points[x][y] = temp_val;
                    // println!("mapped value horizontal {}",self.mapped_points[x][y]);
                }
            }
        }
    }
    pub fn draw_on_map_diagonal(&mut self, input: &Coord) {
        // Draws the diagonal lines.
        if input.c1.0 != input.c2.0 && input.c1.1 != input.c2.1 {
            // Basic Diagonal test. no X or Y has the same value between pairs.
            if input.c1.0 < input.c2.0 && input.c1.1 < input.c2.1 {
                // Ascending X, Ascending Y. Line runs NW -> SE
                for index in 0..=(input.c2.0 - input.c1.0) {
                    let x = input.c1.0 + index;
                    let y = input.c1.1 + index;
                    let mut temp_val: usize = self.mapped_points[x][y];
                    temp_val += 1;
                    self.mapped_points[x][y] = temp_val;
                    // println!("mapped value diagonal coords: \
                    //     {},{} value: {}",x,y,self.mapped_points[x][y]);
                }
            }
            if input.c1.0 > input.c2.0 && input.c1.1 < input.c2.1 {
                // Descending X, Ascending Y. Line runs NE -> SW
                for index in 0..=(input.c1.0 - input.c2.0) {
                    let x = input.c1.0 - index;
                    let y = input.c1.1 + index;
                    let mut temp_val: usize = self.mapped_points[x][y];
                    temp_val += 1;
                    self.mapped_points[x][y] = temp_val;
                }
            }
            if input.c1.0 < input.c2.0 && input.c1.1 > input.c2.1 {
                // Ascending X, Descening Y. Line runs SW -> NE
                for index in 0..=(input.c2.0 - input.c1.0) {
                    let x = input.c1.0 + index;
                    let y = input.c1.1 - index;
                    let mut temp_val: usize = self.mapped_points[x][y];
                    temp_val += 1;
                    self.mapped_points[x][y] = temp_val;
                }
            }
            if input.c1.0 > input.c2.0 && input.c1.1 > input.c2.1 {
                // Descending X, Descending Y. Line runs SE -> NW
                for index in 0..=(input.c1.0 - input.c2.0) {
                    let x = input.c1.0 - index;
                    let y = input.c1.1 - index;
                    let mut temp_val: usize = self.mapped_points[x][y];
                    temp_val += 1;
                    self.mapped_points[x][y] = temp_val;
                }
            }
        }
    }
}

pub fn day5_challenge1(config: &Config) -> Result<i32, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let mut sea_map = Mapper::new().unwrap();
    let mut input_integers: Vec<usize> = Vec::new();
    for line in lines.iter() {
        // parse the input lines.  Splits the input, filters and parses only for integers and
        // builds a vector.
        // This is a great personal success to write this code without finding direct examples.
        input_integers = line
            .split(|s| !char::is_numeric(s))
            .filter_map(|s| str::parse::<usize>(s).ok())
            .collect::<Vec<usize>>();
        let one_coord = Coord::add(
            input_integers[0],
            input_integers[1],
            input_integers[2],
            input_integers[3],
        );
        sea_map.draw_on_map(&one_coord);
    }
    let mut count_2_plus: usize = 0;
    for points in sea_map.mapped_points.iter() {
        if points > &1 {
            count_2_plus += 1;
        }
    }
    Ok(count_2_plus as i32)
}

pub fn day5_challenge2(config: &Config) -> Result<i32, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let mut sea_map = Mapper::new().unwrap();
    let mut input_integers: Vec<usize> = Vec::new();
    for line in lines.iter() {
        // parse the input lines.  Splits the input, filters and parses only for integers and
        // builds a vector.
        // This is a great personal success to get to this code finding direct examples.
        input_integers = line
            .split(|s| !char::is_numeric(s))
            .filter_map(|s| str::parse::<usize>(s).ok())
            .collect::<Vec<usize>>();
        println!("{:?}", input_integers);
        let one_coord = Coord::add(
            input_integers[0],
            input_integers[1],
            input_integers[2],
            input_integers[3],
        );
        // This is where it starts to change from the previous challenge
        sea_map.draw_on_map(&one_coord);
        sea_map.draw_on_map_diagonal(&one_coord);
    }
    // println!("{:?}",sea_map.mapped_points);
    // for (index, line) in integers_only.iter().enumerate() {
    //     sea_map.draw_on_map(sea_map,index);
    // }
    let mut count_2_plus: usize = 0;
    for points in sea_map.mapped_points.iter() {
        if points > &1 {
            count_2_plus += 1;
        }
    }
    println!("2 overlapped {}", count_2_plus);
    Ok(count_2_plus as i32)
}
