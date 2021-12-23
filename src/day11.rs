// Advent of Code Day ELEVEN
//
// Some imports not needed every time.
use a_o_c::*; //import custom lib.rs module
use grid::*;
use std::io::Error;
// use std::process;
const GRID_X_MAX: usize = 10;
const GRID_Y_MAX: usize = 10;
const MAX_INDEX_RIGHT: usize = GRID_Y_MAX - 1;
const MAX_INDEX_BOTT: usize = GRID_X_MAX - 1;

pub struct OGrid {
    grid: Grid<isize>,
    flash_grid: Grid<bool>,
    flashes_step: usize,
}
impl OGrid {
    pub fn new() -> OGrid {
        let grid = Grid::new(GRID_Y_MAX, GRID_X_MAX);
        let flash_grid = Grid::init(GRID_Y_MAX, GRID_X_MAX, false);
        let flashes_step = 0;
        OGrid {
            grid,
            flash_grid,
            flashes_step,
        }
    }
    pub fn increase_energy_full(&mut self) {
        for pod in self.grid.iter_mut() {
            *pod += 1;
        }
    }
    pub fn evaluate(&mut self) {
        // Goes through all steps to evaluate whether a flash has occured and increases energy
        // levels in adjacent spaces.
        self.flashes_step = 0;
        for row in 0..GRID_Y_MAX {
            for col in 0..GRID_X_MAX {
                // perform recursive evalation of all fired points and surrounding points.
                // self.flashes_step +=
                recurse_eval(&mut self.grid, &mut self.flash_grid, (row, col));
            }
        }
        // set all points that flashed to 0
        for row in 0..GRID_Y_MAX {
            for col in 0..GRID_X_MAX {
                // if self.grid[row][col] > 9 {
                if self.flash_grid[row][col] {
                    self.grid[row][col] = 0;
                    self.flashes_step += 1;
                }
            }
        }
        //reset the flash tracking matrix
        self.flash_grid = Grid::init(GRID_Y_MAX, GRID_X_MAX, false);

        fn recurse_eval(
            recurse_grid: &mut Grid<isize>,
            flash_grid: &mut Grid<bool>,
            row_col: (usize, usize),
        ) -> usize {
            // recursive function to evaluate a 3x3 grid or points based on the center coordinate of
            // row, column passed.
            // returns a vector of coordinates that still must be evaluated based on results of the
            // operation.
            // This is not very efficent as it constructs and destructs variables for each run which
            // occurs at each grid element.
            // println!("recurse_eval started for coordinates {:?}", row_col);
            if recurse_grid[row_col.0][row_col.1] < 10 || flash_grid[row_col.0][row_col.1] {
                // base recursion case. Less than the flash energy or already flashed.
                // Nothing to do
                return 0;
            } else {
                // condition when a flash has occured.
                // log flash of tested coordinate.
                let mut flash_counter: usize = 1;
                flash_grid[row_col.0][row_col.1] = true;
                // grid[row_col.0][row_col.1] = -1;
                // prepare for evaluation of all surrounding coordinates.
                let mut next_eval_vec: Vec<(usize, usize)> = Vec::new();
                // sectors:     1 2 3
                //              4 X 6
                //              7 8 9
                //
                // assign sector values.  The use of wrapping subtraction is meant to prevent
                // panics and reduce the need for error handlin here for bounded computation.
                // Wrapped values are dealt with in the following section where border cases are
                // tested and set to values of None.
                let mut sector_1 = Some((row_col.0.wrapping_sub(1), row_col.1.wrapping_sub(1)));
                let mut sector_2 = Some((row_col.0.wrapping_sub(1), row_col.1));
                let mut sector_3 = Some((row_col.0.wrapping_sub(1), row_col.1 + 1));
                let mut sector_4 = Some((row_col.0, row_col.1.wrapping_sub(1)));
                // let sector_X = Some((row_col.0, row_col.1));
                let mut sector_6 = Some((row_col.0, row_col.1 + 1));
                let mut sector_7 = Some((row_col.0 + 1, row_col.1.wrapping_sub(1)));
                let mut sector_8 = Some((row_col.0 + 1, row_col.1));
                let mut sector_9 = Some((row_col.0 + 1, row_col.1 + 1));

                if row_col.0 == 0 {
                    // test for top row
                    sector_1 = None;
                    sector_2 = None;
                    sector_3 = None;
                }
                if row_col.0 == MAX_INDEX_BOTT {
                    // test for bottom row
                    sector_7 = None;
                    sector_8 = None;
                    sector_9 = None;
                }
                if row_col.1 == 0 {
                    // test for left side
                    sector_1 = None;
                    sector_4 = None;
                    sector_7 = None;
                }
                if row_col.1 == MAX_INDEX_RIGHT {
                    // test for right side
                    sector_3 = None;
                    sector_6 = None;
                    sector_9 = None;
                }

                let mut increment_test = |r_c: (usize, usize)| {
                    // increments the level of the point and tests if this is also reaching the
                    // threshold to increment the function flash counter and push to the vector to
                    // further test.
                    recurse_grid[r_c.0][r_c.1] += 1;

                    if flash_grid[r_c.0][r_c.1] {
                        // If it has already flashed (value set to true), do nothing else.
                    }
                    // println!("increment test {:?}", r_c);
                    else if recurse_grid[r_c.0][r_c.1] > 9 {
                        // New flash
                        next_eval_vec.push(r_c);
                        flash_counter += 1;
                    } else {
                        // no flash. increment performed already.
                    }
                };
                for sector in [
                    sector_1, sector_2, sector_3, sector_4, sector_6, sector_7, sector_8, sector_9,
                ]
                .into_iter()
                {
                    // if let sector_r_c = sector.unwrap_or_else(||{continue;});
                    if sector == None {
                        continue;
                    } else {
                        increment_test(sector.unwrap());
                    }
                }
                // let mut work_vec: Vec<(usize,usize)> = Vec::new();
                // Here's the recursive portion.
                // println!("next_eval_vec: {:?}",next_eval_vec);
                for coord in next_eval_vec.iter() {
                    flash_counter += recurse_eval(recurse_grid, flash_grid, *coord);
                }

                flash_counter
            }
        }
    }
}

pub fn day11_challenge1(config: &Config) -> Result<i128, Error> {
    // Create grid data structure for data input. Default value initialized high to detect errors.
    // let mut grid_map: Grid<usize> = Grid::init(GRID_Y_MAX, GRID_X_MAX, usize::MAX);
    // Create grid data structure for marking low points. Zero represents no mark.
    let mut octos = OGrid::new();
    // let mut low_point_map: Grid<bool> = Grid::init(GRID_X_MAX, GRID_Y_MAX, false);
    // create variables for tracking totals and score.
    let mut total_flashes: usize = 0;
    // let mut total_score: usize = 0;
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    for (row, line) in lines.iter().enumerate() {
        for (col, digit) in line.chars().enumerate() {
            octos.grid[row][col] = digit.to_string().parse::<isize>().unwrap_or(1000);
        }
    }
    println!("\n\n ---- CHALLENGE ONE ---- \n\n");
    for step in 1..=100 {
        octos.increase_energy_full();
        octos.evaluate();
        total_flashes += octos.flashes_step;
        if step < 11 {
            println!(
            "\n\nFirst 10 steps... For step {}\nFlashes this step: {} Total: {}\n octos grid map\n {:?}",
            step, octos.flashes_step, total_flashes, octos.grid
        );
        }
    }
    println!(
        "\n\nFinal step... Flashes this step: {} Total: {}\n octos grid map\n {:?}",
        octos.flashes_step, total_flashes, octos.grid
    );
    Ok(total_flashes as i128)
}

pub fn day11_challenge2(config: &Config) -> Result<i128, Error> {
    // Create grid data structure for data input. Default value initialized high to detect errors.
    // let mut grid_map: Grid<usize> = Grid::init(GRID_Y_MAX, GRID_X_MAX, usize::MAX);
    // Create grid data structure for marking low points. Zero represents no mark.
    let mut octos = OGrid::new();
    // let mut low_point_map: Grid<bool> = Grid::init(GRID_X_MAX, GRID_Y_MAX, false);
    // create variables for tracking totals and score.
    let mut total_flashes: usize = 0;
    // let mut total_score: usize = 0;
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    for (row, line) in lines.iter().enumerate() {
        for (col, digit) in line.chars().enumerate() {
            octos.grid[row][col] = digit.to_string().parse::<isize>().unwrap_or(1000);
        }
    }
    let mut result: usize = 0;
    let mut step: usize = 0;
    println!("\n\n ---- CHALLENGE TWO ---- \n\n");
    while result == 0 {
        step += 1;
        // println!("\n\nStarting Step {}",step);
        octos.increase_energy_full();
        octos.evaluate();
        total_flashes += octos.flashes_step;
        if step < 11 {
            println!(
            "\n\nFirst 10 steps... For step {}\nFlashes this step: {} Total: {}\n octos grid map\n {:?}",
            step, octos.flashes_step, total_flashes, octos.grid
        );
        }
        if octos.flashes_step == 100 {
            println!(
            "\n\nFinal step... For step {}\nFlashes this step: {} Total: {}\n octos grid map\n {:?}",
            step, octos.flashes_step, total_flashes, octos.grid
        );
            result = step;
            break;
        }
    }
    // fn increase_energy_level(self) {}
    // println!("total number of flashes: {}", total_flashes);
    Ok(result as i128)
}
