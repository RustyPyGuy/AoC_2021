// Advent of Code Day TEMPLATE
//
// Some imports not needed every time.
use a_o_c::*;  //import custom lib.rs module
// use std::fs::File;
// use std::error::Error;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
// use std::process;

#[derive(Debug, Copy, Clone)]
pub struct DataStruct {
x: i32, 
y: i32,
z: i32,
}

impl DataStruct {
    pub fn new() -> Result<DataStruct, &'static str>{
        let x = 0;
        let y = 0;
        let z = 0;
        Ok(DataStruct{x,y,z,})
    }
    pub fn export(self) -> Result<DataStruct, &'static str> {
        // return Ok(Coordinates{self.x,self.y,self.z});
        println!("export method called returning struct {:?}",self);
        Ok(self)
    }
}

pub fn day0_challenge1(config: &Config) -> Result<i32, Error> {
    let mut data_struct = DataStruct::new().unwrap();
    perform_calculations(config, &mut data_struct);
    println!("interim caluculations {:?}",data_struct);
    // final calculations below
    Ok(data_struct.x*data_struct.y)
}

pub fn day0_challenge2(config: &Config) -> Result<i32, Error> {
    let mut data_struct = DataStruct::new().unwrap();
    perform_calculations(config, &mut data_struct);
    println!("interim caluculations {:?}",data_struct);
    // final calculations below
    Ok(data_struct.x*data_struct.y)
}
