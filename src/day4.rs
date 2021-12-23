// Advent of Code Day Day 4
//
// Some imports not needed every time.
use a_o_c::*; //import custom lib.rs module
use grid::*;
use std::io::Error; //using a custom crate for once.

pub fn day4_challenge1(config: &Config) -> Result<i32, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let called_numbers = parse_string_line_into_integers(lines.get(0).unwrap().to_string(), ',');
    // build some bingo cards
    let mut bingo_cards: Vec<BingoCard> = Vec::new();
    // read in 5 lines at a time
    for five_lines in lines[1..].chunks(5) {
        let mut working_bingo_card = BingoCard::new().unwrap();
        // build row by row since that's how the BingoCard method is written
        // Use the Rust feature of enumerating over a vector, which provide an iterable and an
        // index.
        for (index, line) in five_lines.iter().enumerate() {
            let row_int: Vec<u32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            working_bingo_card.build_card_by_row(row_int, index);
        }
        bingo_cards.push(working_bingo_card);
    }
    let mut is_winner: bool = false;
    let mut winning_score: u32 = 0;
    let mut sum_unmarked: u32 = 0;
    println!("number of bingo cards: {}", bingo_cards.len());
    for called_number in &called_numbers {
        // for one_card in bingo_cards.iter_mut() {
        for (index, one_card) in bingo_cards.iter_mut().enumerate() {
            is_winner = one_card.evaluate_one_called_number(*called_number);
            if is_winner {
                println!(
                    "we have a winner! number: {}\ncard index number:{}\n{:?}",
                    called_number, index, one_card
                );
                let temp_vec = one_card.unmarked_numbers();
                for number in temp_vec.iter() {
                    sum_unmarked += number;
                }
                winning_score = sum_unmarked * called_number;
                break;
            }
        }

        if is_winner {
            break;
        }
    }
    Ok(winning_score as i32)
}

pub fn day4_challenge2(config: &Config) -> Result<i32, Error> {
    // read input puzzle file
    let input_string = read_prep_puzzle_file_contents_to_string(config);
    // read called numbers all on first line and assign to a vector.
    let lines = iterate_by_lines_from_string(&input_string);
    let called_numbers = parse_string_line_into_integers(lines.get(0).unwrap().to_string(), ',');
    // build some bingo cards
    let mut bingo_cards: Vec<BingoCard> = Vec::new();
    // read in 5 lines at a time
    for five_lines in lines[1..].chunks(5) {
        let mut working_bingo_card = BingoCard::new().unwrap();
        // build row by row since that's how the BingoCard method is written
        // Use the Rust feature of enumerating over a vector, which provide an iterable and an
        // index.
        for (index, line) in five_lines.iter().enumerate() {
            let row_int: Vec<u32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            working_bingo_card.build_card_by_row(row_int, index);
        }
        bingo_cards.push(working_bingo_card);
    }
    for called_number in &called_numbers {
        for (_index, one_card) in bingo_cards.iter_mut().enumerate() {
            // run all the numbers through all the cards. We check which is worst next.
            one_card.evaluate_one_called_number(*called_number);
        }
    }
    let mut worst_card_index: usize = 0;
    let mut worst_card_winning_number_index: usize = 0;
    for (index, one_card) in bingo_cards.iter_mut().enumerate() {
        // go through all the cards and see which one won last
        if worst_card_winning_number_index <= one_card.first_win_index {
            worst_card_winning_number_index = one_card.first_win_index;
            worst_card_index = index;
        }
    }
    bingo_cards[worst_card_index].set_all_numbers_unmarked();
    let mut sum_unmarked: u32 = 0;
    for called_number in &called_numbers[0..worst_card_winning_number_index + 1] {
        bingo_cards[worst_card_index].evaluate_one_called_number(*called_number);
    }
    let temp_vec = bingo_cards[worst_card_index].unmarked_numbers();
    for number in temp_vec.iter() {
        sum_unmarked += number;
    }
    let worst_card_winning_score = called_numbers[worst_card_winning_number_index] * sum_unmarked;

    println!(
        "we have a SLOW winner! number: {} (index {})\ncard index number:{}\n{:?}",
        called_numbers[worst_card_winning_number_index],
        worst_card_winning_number_index,
        worst_card_index,
        bingo_cards[worst_card_index].card
    );
    Ok(worst_card_winning_score as i32)
}

#[derive(Debug, Clone)]
pub struct BingoCard {
    card: Grid<MarkedNumber>, // Contains a vector grid of enums that indicate whether a number is marked.
    marked_numbers: Vec<u32>, // A vector that only contains marked numbers (somewhat redundant, but possibly useful)
    // card_complete: bool, // Stores state that the card is completely populated, again redundant.
    winning_exists: bool, // Stores state that the card is a winner based on marked numbers.
    winning_input_index: u32, // Index of numbers called.  The number of numbers ingested for possible marking.
    first_win_index: usize, // The index of the input numbers that first triggers a win.  Upon winning, will not change unless the card is cleared.
}
impl BingoCard {
    pub fn new() -> Result<BingoCard, &'static str> {
        let card = Grid::new(5, 5);
        let marked_numbers = Vec::new();
        // let card_complete = false;
        let winning_exists = false;
        let winning_input_index = 0;
        let first_win_index = 0;
        Ok(BingoCard {
            card,
            marked_numbers,
            // card_complete,
            winning_exists,
            winning_input_index,
            first_win_index,
        })
    }
    // pub fn init() -> Result<BingoCard, &'static str> {
    //     let card = Grid::init(5, 5, MarkedNumber::None);
    //     let marked_numbers = Vec::new();
    //     // let card_complete = false;
    //     let winning_exists = false;
    //     let winning_input_index = 0;
    //     let first_win_index = 0;
    //     Ok(BingoCard {
    //         card,
    //         marked_numbers,
    //         // card_complete,
    //         winning_exists,
    //         winning_input_index,
    //         first_win_index,
    //     })
    // }
    pub fn build_card_by_row(&mut self, input: Vec<u32>, row: usize) {
        let mut temp_row: Vec<MarkedNumber> = Vec::new();
        for number in input.iter() {
            temp_row.push(MarkedNumber::UnMarked(*number));
        }
        self.card.insert_row(row, temp_row);
        self.card.pop_row();
    }
    pub fn evaluate_one_called_number(&mut self, called_number: u32) -> bool {
        // THIS IS THE BUSINESS.  This is the heart of the program.
        // Start by looping through the elements and marking the called number.
        //my orignial plan was to use an iterator, but that got to complicated for the
        //reassignment of the row, column index. Sometimes old tried ways are the best.
        for row in 0..5 {
            for col in 0..5 {
                if self.card[row][col].clone().unwrap() == called_number {
                    self.card[row][col] = MarkedNumber::Marked(called_number);
                    self.marked_numbers.push(called_number);
                }
            }
        }
        // Increment the winning input index whether or not there's a win. This helps
        // associate the index of the winning number.
        self.winning_input_index += 1;
        // evaluate if there's a winner
        // Go through the rows and columns and see if all have the correct enum value
        // Rows first
        for selected_row in 0..self.card.rows() {
            self.winning_exists = true; // set the winning flag. It is assumed true to prepare logic comparisons.
            for element in self.card.iter_row(selected_row) {
                if std::mem::discriminant(element)
                    == std::mem::discriminant(&MarkedNumber::Marked(u32::MAX))
                {
                    self.winning_exists &= true; // assign self logic AND true
                    continue; // next iteration if true. If last element is true, exits loop here.
                }
                // if any number in the row is not marked this portion is executed.
                // The winning flag is set to false.  One false value will stick.
                self.winning_exists &= false; // assign self logic AND false.
            }
            // If there is a winner with this row, stop evaluating and preserve the flag.
            if self.winning_exists {
                if self.first_win_index == 0 {
                    // note the first index of winning numbers.
                    self.first_win_index = (self.winning_input_index - 1) as usize;
                }
                return self.winning_exists;
            } // true
        }
        // Columns next.
        for selected_column in 0..self.card.cols() {
            self.winning_exists = true; // set the winning flag. It is assumed true to prepare logic comparisons.
            for element in self.card.iter_col(selected_column) {
                if std::mem::discriminant(element)
                    == std::mem::discriminant(&MarkedNumber::Marked(u32::MAX))
                {
                    self.winning_exists &= true; // assign self logic AND true
                    continue; // next iteration if true.
                }
                // if any number in the row is not marked this portion is executed.
                // The winning flag is set to false.  One false value will stick.
                self.winning_exists &= false; // assign self logic AND false.
            }
            // If there is a winner with this column, stop evaluating and preserve the flag.
            if self.winning_exists {
                if self.first_win_index == 0 {
                    // note the first index of winning numbers.
                    self.first_win_index = (self.winning_input_index - 1) as usize;
                }
                return self.winning_exists;
            } // true
        }
        return self.winning_exists;
    }
    pub fn unmarked_numbers(&self) -> Vec<u32> {
        let mut unmarked_numbers_list: Vec<u32> = Vec::new();
        for element in self.card.iter() {
            if std::mem::discriminant(element)
                == std::mem::discriminant(&MarkedNumber::UnMarked(u32::MAX))
            {
                unmarked_numbers_list.push(element.clone().unwrap());
            }
        }
        unmarked_numbers_list
    }
    pub fn set_all_numbers_unmarked(&mut self) {
        for row in 0..5 {
            for col in 0..5 {
                let temp_num = self.card[row][col].clone().unwrap();
                self.card[row][col] = MarkedNumber::UnMarked(temp_num);
            }
        }
    }

    // pub fn export(self) -> Result<BingoCard, &'static str> {
    //     println!("export method called returning struct {:?}", self);
    //     Ok(self)
    // }
}
#[derive(PartialEq, Eq, Debug, Clone)]
enum MarkedNumber {
    Marked(u32),
    UnMarked(u32),
    None,
}
impl Default for MarkedNumber {
    fn default() -> Self {
        MarkedNumber::None
    }
}
impl MarkedNumber {
    fn unwrap(self) -> u32 {
        match self {
            MarkedNumber::Marked(val) => val,
            MarkedNumber::UnMarked(val) => val,
            MarkedNumber::None => u32::MAX,
            // Since this has to return a number. The only alternative is to panic.
            //      Douglas Adams says "Don't Panic!"
            //  The Rust way of doing would be to return an enum Result or
            //  Option, but that would have to be unwrapped again. I'd prefer not
            //  to do that right now.
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_test_card() -> BingoCard {
        let mut test_board: BingoCard = BingoCard::new().unwrap();
        for i in 0..5 {
            let fake_vec: Vec<u32> =
                vec![i * 10 + 1, i * 10 + 2, i * 10 + 3, i * 10 + 4, i * 10 + 5];
            test_board.build_card_by_row(fake_vec, i as usize);
        }
        test_board
    }

    #[test]
    fn build_one_board() {
        let mut board: BingoCard = BingoCard::new().unwrap();
        // let fake_vec: Vec<MarkedNumber> = vec![MarkedNumber::UnMarked(1);5];
        for i in 0..5 {
            let fake_vec: Vec<u32> =
                vec![i * 10 + 1, i * 10 + 2, i * 10 + 3, i * 10 + 4, i * 10 + 5];
            board.build_card_by_row(fake_vec, i as usize);
        }
        println!("board {:?}", board.card);
        // self.card.insert_row(row,fake_vec );
        let test_col = board.card.pop_col().unwrap();
        assert_eq!(
            test_col,
            vec![
                MarkedNumber::UnMarked(5),
                MarkedNumber::UnMarked(15),
                MarkedNumber::UnMarked(25),
                MarkedNumber::UnMarked(35),
                MarkedNumber::UnMarked(45)
            ]
        );
    }

    #[test]
    fn winning_number_flagged() {
        let mut board = build_test_card();
        let mut winner: bool = false;
        winner ^= board.evaluate_one_called_number(5); //exclusive or, XOR
        winner ^= board.evaluate_one_called_number(15);
        winner ^= board.evaluate_one_called_number(25);
        winner ^= board.evaluate_one_called_number(27);
        winner ^= board.evaluate_one_called_number(11);
        winner ^= board.evaluate_one_called_number(35);
        winner ^= board.evaluate_one_called_number(35); //no winner
        winner ^= board.evaluate_one_called_number(99); //no winner
        winner ^= board.evaluate_one_called_number(45); //should trigger winner
        assert!(winner);
    }
}
