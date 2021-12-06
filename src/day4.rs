// Advent of Code Day TEMPLATE
//
// Some imports not needed every time.
use a_o_c::*;  //import custom lib.rs module
// use std::fs::File;
// use std::error::Error;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
// use std::process;
use grid::*;

#[derive(Debug, Clone)]
pub struct BingoCard {
// card: Grid<u32>, 
card: Grid<MarkedNumber>, // Contains a vector grid of enums that indicate whether a number is marked.
// winning_groups: Vec<Vec<u32>>,
marked_numbers: Vec<u32>, // A vector that only contains marked numbers (somewhat redundant, but possibly useful)
card_complete: bool,  // Stores state that the card is completely populated, again redundant.
winning_exists: bool, // Stores state that the card is a winner based on marked numbers. 
winning_input_index: u32, // The number of numbers ingested for possible marking.
}
impl BingoCard {
    pub fn new() -> Result<BingoCard, &'static str>{
        let card = Grid::new(5,5);
        // winning_groups: <Vec<u32>>,
        let marked_numbers = Vec::new();
        let card_complete = false;
        let winning_exists = false;
        let winning_input_index = 0;
        Ok(BingoCard{card,marked_numbers,card_complete,winning_exists,winning_input_index})
    }
    pub fn build_card_by_row(mut self, input: Vec<u32>, row: usize) {
        let mut temp_row: Vec<MarkedNumber> = Vec::new();
        for number in input.iter(){
            temp_row.push(MarkedNumber::UnMarked(*number)/*BingoCard::new_enum_unmarked_number(*number)*/);
        }
        self.card.insert_row(row, temp_row);
    }
    fn new_enum_unmarked_number(input: u32) -> MarkedNumber {
       MarkedNumber::UnMarked(input)
    }
    pub fn evaluate_one_called_number(mut self, called_number: u32){
        // THIS IS THE BUSINESS
        // Start by looping through the elements and marking the called number.
            for mut card_number in self.card.clone().iter() {
                if card_number.clone().unwrap() == called_number {
                    card_number = &MarkedNumber::Marked(called_number);
                    // self.card = card_number;
                    // let new_card_number = &MarkedNumber::Marked(*called_number); 
                    // Add this number to the list of called and marked numbers.
                    // NOTE: Need to check if this will act as designed.
                    self.marked_numbers.push(called_number);
                } 
            }
             // Increment the winning input index whether or not there's a win
            self.winning_input_index +=1;
            // evaluate if there's a winner
            // Go through the rows and columns and see if all have the correct enum value
            // Rows first
            for selected_row in 0..self.card.rows() {
                for element in self.card.iter_row(selected_row){
                    if std::mem::discriminant(element) == 
                        std::mem::discriminant(&MarkedNumber::Marked(u32::MAX)) {
                       self.winning_exists = true;  // NOTE: Broken compare logic need to fix.
                       continue; // next iteration if true.
                    }
                    // if any number in the row is not marked, we reset the winning marker. 
                   self.winning_exists = false; 
                }
            }
            // since we have a winner, stop evaluating.
            if self.winning_exists {return;} // true
            // Columns next.
    }
    pub fn export(self) -> Result<BingoCard, &'static str> {
        // return Ok(Coordinates{self.x,self.y,self.z});
        println!("export method called returning struct {:?}",self);
        Ok(self)
    }
}
#[derive(PartialEq, Eq, Debug,  Clone)]
enum MarkedNumber {
    Marked(u32),
    UnMarked(u32),
    None,
}
// pub trait Default {
//     fn summarize(&self) -> String;
// }
impl Default for MarkedNumber {
    fn default() -> Self { MarkedNumber::None }
}
impl MarkedNumber {
    fn unwrap(self) -> u32 { 
    match self {
        MarkedNumber::Marked(val) => val,
        MarkedNumber::UnMarked(val) => val,
        MarkedNumber::None => u32::MAX, // Since this has to return a number. The only alternative is to panic.
                                        //      Douglass Adams says "Don't Panic!"
                                        //  The Rust way of doing would be to return an enum Result or
                                        //  Option, but that would have to be unwrapped again. I'd prefer not
                                        //  to do that right now.
        }
    }
}

pub fn day4_challenge1(config: &Config) -> Result<i32, Error> {
    // let mut data_struct = DataStruct::new().unwrap();
    // perform_calculations(config, &mut data_struct);
    // println!("interim caluculations {:?}",data_struct);
    // final calculations below
    // Ok(data_struct.x*data_struct.y)
    Ok(0)
}

pub fn day4_challenge2(config: &Config) -> Result<i32, Error> {
    // let mut data_struct = DataStruct::new().unwrap();
    // perform_calculations(config, &mut data_struct);
    // println!("interim caluculations {:?}",data_struct);
    // final calculations below
    // Ok(data_struct.x*data_struct.y)
    Ok(0)
}
