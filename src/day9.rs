// Advent of Code Day TODAY
//
// Some imports not needed every time.
use a_o_c::*; //import custom lib.rs module
use grid::*;
use std::io::Error;
// use std::process;
const GRID_X_MAX: usize = 10;
const GRID_Y_MAX: usize = 5;

pub fn day9_challenge1(config: &Config) -> Result<i128, Error> {
    // Create grid data structure for data input. Default value initialized high to detect errors.
    let mut grid_map: Grid<usize> = Grid::init(GRID_Y_MAX,GRID_X_MAX,usize::MAX);
    // Create grid data structure for marking low points. Zero represents no mark.
    let mut low_point_map: Grid<bool>= Grid::init(GRID_X_MAX+2,GRID_Y_MAX+2,false);
    // create variables for tracking totals and score.
    let mut sum_low_points: usize = 0;
    let mut total_score: usize = 0;
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    for (row,line) in lines.iter().enumerate() {
        for (col,digit) in line.chars().enumerate(){
            grid_map[row][col]=digit.to_string().parse::<usize>().unwrap_or(55);
        } 
    }
    grid_map.insert_row(0,vec![66;GRID_X_MAX]);
    grid_map.insert_col(0,vec![77;GRID_Y_MAX+1]); //processor expensive
    grid_map.push_row(vec![88;GRID_X_MAX+1]);
    grid_map.push_col(vec![99;GRID_Y_MAX+2]); //processor expensive
    println!("grid map {:?}",grid_map);
    println!("grid map size {:?}",grid_map.size());

    // go through each element and compare one element above and right to determine if the compared
    // element is the lowest.  The border of high numbers reduces the need to account for boundary
    // conditions
    // for (index, element) in grid_map.iter().enumerate(){
    //     if element < &grid_map[index] && element < &grid_map[index+grid_map.cols()] {
    //        low_point_map[index] = element; 
    //     }
    // }
    for row in 1..grid_map.rows()-1 {
        for col in 1..grid_map.cols()-1 {
           if grid_map[row][col] < grid_map[row+1][col] &&
               grid_map[row][col] < grid_map[row][col+1] && 
           grid_map[row][col] < grid_map[row-1][col] &&
               grid_map[row][col] < grid_map[row][col-1] {
                   low_point_map[row][col] = true; 
                   println!("position, {}, {}: {}",row,col,grid_map[row][col]);
                sum_low_points +=1;
                // let temp: usize = *grid_map[index].first().unwrap_or(&0);
                // total_score += temp+1_usize;
                println!("score bump {}", grid_map[row][col]+1);
                total_score += grid_map[row][col]+1_usize;
           }
        }
    }
    println!("low point map {:?}",low_point_map);
    println!("low point map size {:?}",low_point_map.size());
    // for (index, point) in low_point_map.iter().enumerate() {
    //     if *point ==1 { sum_low_points +=1;
    //         let temp: usize = *grid_map[index].first().unwrap_or(&0);
    //         total_score += temp+1_usize;
    //     }
    // }
    // for row in 1..low_point_map.rows()-1 {
    //     for col in 1..low_point_map.cols()-1 {

    //     if low_point_map[row][col]==true { sum_low_points +=1;
    //         // let temp: usize = *grid_map[index].first().unwrap_or(&0);
    //         // total_score += temp+1_usize;
    //         println!("score bump {}", grid_map[row][col]+1);
    //         total_score += grid_map[row][col]+1_usize;
    //         }
    //     }
    // }
    println!("total number of low points: {}",sum_low_points);
    Ok(total_score as i128)
}

pub fn day9_challenge2(config: &Config) -> Result<i128, Error> {
    Ok(0 as i128)
}
