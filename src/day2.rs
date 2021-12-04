// Day 2 Advent of code

use a_o_c::*;
use std::fs::File;
// use std::error::Error;
use std::io::{BufRead, BufReader, Error, Read};
// use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::process;
// Store coordinates from origin     
#[derive(Debug, Copy, Clone)]
struct Coordinates {
x: i32, // X: (+forward -back) 
y: i32, // Y: (+right -left)
z: i32, // Z: (+up -down)
}
impl Coordinates {
    pub fn new(self) -> Result<Coordinates, &'static str>{
        let x = 0;
        let y = 0;
        let z = 0;
        return Ok(Coordinates{x,y,z});
    }
    pub fn move_x(mut self, val: i32){
        self.x = self.x +val;
    }
    pub fn move_y(mut self, val: i32){
        self.y = self.y +val;
    }
    pub fn move_z(mut self, val: i32){
        self.z = self.z +val;
    }
}

fn read_by_lines_double<'a, R: Read>(io: R, coord: &'a Coordinates) -> Result<&Coordinates, std::io::Error> {
    let br = BufReader::new(io);
    for line in br.lines() {
            if let Ok(field) = line {
            let mut fields = field.split_whitespace();
           let field1 = fields.next();
            // Silly thing keeps making Results and options that need to be unwrapped twice.  This
            // piece really got me stuck for a while.
           let field2: i32 = fields.next().unwrap().parse::<i32>().unwrap(); 
            match field1 {
                Some("forward") => {coord.move_x(field2)},
                Some("back") => {coord.move_x(field2)},
                Some("left") => {coord.move_x(field2)},
                Some("right") => {coord.move_x(field2)},
                Some("up") => {coord.move_x(field2)},
                Some("down") => {coord.move_x(field2)},
                Some(_) => {println!("Failure with word pattern match.");},
                None => {println!("General failure with word parsing");},
            }
            
        }
            // Err({println!("Error reading line");});
    }

    // for line in br.lines() {
    //     // if let Ok(fields) = line {line?.split_whitespace();  };
    //     let mut fields = line?.split_whitespace();
    //     let field1 = &fields.next();
    //     let field2 = fields.next();
    //     *match field1 {
    //         Some("forward") => &coord.move_x(line?.parse::<i32>().unwrap()),
    //         // Some("back") => &coord.move_x(-line?.parse::<i32>().unwrap()),
    //         // Some("up") => &coord.move_z(line?.parse::<i32>().unwrap()),
    //         // Some("down") => &coord.move_z(line?.parse::<i32>().unwrap()),
    //         Some(_) => &{println!("word pattern match fail.");},
    //         None => &{println!("fail word parse");},
    //         // Err => {println!("failed with an internal error at word parse");},
    //     }
    //     // for word in v
    //     //         .trim()
    //     //         .split_whitespace() {
    //     //             match word {
    //     //                 _ => (),
    //     //             }
    //     //         };
            
    // }
    Ok(&coord)
}



pub fn day2_challenge1(config: Config) -> Result<u32, Error> {
    let f = File::open(config.filename).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    // for item in contents.into_iter() {
    

    // }
    // let contents = read_by_lines(f).expect("failed");
    Ok(0)
}
