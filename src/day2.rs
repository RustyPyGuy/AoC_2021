// Day 2 Advent of code
use a_o_c::*;
use std::fs::File;
// use std::error::Error;
use std::io::{BufRead, BufReader, Error, Read};
// use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::process;
// Store coordinates from origin

#[derive(Debug, Copy, Clone)]
pub struct Coordinates {
    x: i32, // X: (+forward -back)
    y: i32, // Y: (+right -left)
    z: i32, // Z: (+down -up)
    aim: i32,
}

impl Coordinates {
    pub fn new() -> Result<Coordinates, &'static str> {
        let x = 0;
        let y = 0;
        let z = 0;
        let aim = 0;
        Ok(Coordinates { x, y, z, aim })
    }
    pub fn move_x(&mut self, val: i32) {
        self.x += val;
        // println!("move_x called with val: {:?}, new coordinate {:?}",val,self.x);
    }
    pub fn move_y(&mut self, val: i32) {
        self.y += val;
    }
    pub fn move_z(&mut self, val: i32) {
        self.z += val;
    }
    pub fn set_aim_up(&mut self, val: i32) {
        self.aim -= val;
    }
    pub fn set_aim_down(&mut self, val: i32) {
        self.aim += val;
    }
    pub fn move_x_with_aim(&mut self, val: i32) {
        //new math here
        self.x += val;
        self.z += self.aim * val;
        // println!("move_x_with_aim called with val: {:?}, new coordinate {:?}",val,self.x);
    }
    pub fn export(self) -> Result<Coordinates, &'static str> {
        // return Ok(Coordinates{self.x,self.y,self.z});
        println!("export method called returning struct {:?}", self);
        Ok(self)
    }
}

fn read_by_lines_double</*'a, */ R: Read>(
    io: R,
    coord: &mut /*'a */ Coordinates,
) -> Result<&Coordinates, std::io::Error> {
    let br = BufReader::new(io);
    for line in br.lines().flatten() {
        let field = line;
        let mut fields = field.split_whitespace();
        let field1 = fields.next();
        // Silly thing keeps making Results and options that need to be unwrapped twice.  This
        // piece really got me stuck for a while. Of course there must be a better way to
        // do this.
        let field2: i32 = fields.next().unwrap().parse::<i32>().unwrap();
        match field1 {
            Some("forward") => {
                coord.move_x(field2);
            }
            // Some("forward") => {coord.move_x(field2);println!("move x {}",field2);},
            Some("back") => coord.move_x(-field2),
            Some("left") => coord.move_y(field2),
            Some("right") => coord.move_y(-field2),
            Some("up") => {
                coord.move_z(-field2);
            }
            Some("down") => coord.move_z(field2),
            Some(_) => {
                println!("Failure with word pattern match.");
            }
            None => {
                println!("General failure with word parsing");
            }
        }
        // Err({println!("Error reading line");});
    }
    Ok(coord)
}

fn read_by_lines_challenge2</*'a, */ R: Read>(
    io: R,
    coord: &mut /*'a */ Coordinates,
) -> Result<&Coordinates, std::io::Error> {
    let br = BufReader::new(io);
    for line in br.lines().flatten() {
        let field = line;
        let mut fields = field.split_whitespace();
        let field1 = fields.next();
        // Silly thing keeps making Results and options that need to be unwrapped twice.  This
        // piece really got me stuck for a while. Of course there must be a better way to
        // do this.
        let field2: i32 = fields.next().unwrap().parse::<i32>().unwrap();
        match field1 {
            Some("forward") => {
                coord.move_x_with_aim(field2);
            }
            Some("up") => {
                coord.set_aim_up(field2);
            }
            Some("down") => {
                coord.set_aim_down(field2);
            }
            Some(_) => {
                println!("Failure with word pattern match.");
            }
            None => {
                println!("General failure with word parsing");
            }
        }
        // Err({println!("Error reading line");});
    }
    Ok(coord)
}

pub fn day2_challenge1(config: &Config) -> Result<i32, Error> {
    let f = File::open(config.filename.clone()).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    let mut submarine_coordinates: Coordinates = Coordinates::new().unwrap();
    read_by_lines_double(f, &mut submarine_coordinates).expect("challenge 1 file read failure.");
    let results = submarine_coordinates.export().unwrap();
    Ok(results.x * results.z.abs())
}

pub fn day2_challenge2(config: &Config) -> Result<i32, Error> {
    let f = File::open(config.filename.clone()).unwrap_or_else(|err| {
        println!("Error opening file: {}", err);
        process::exit(1);
    });
    let mut submarine_coordinates: Coordinates = Coordinates::new().unwrap();
    read_by_lines_challenge2(f, &mut submarine_coordinates)
        .expect("Challenge 2 file read failure.");
    let results2 = submarine_coordinates.export().unwrap();
    Ok(results2.x * results2.z.abs())
}
