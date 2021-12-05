// Advent of Code Day TEMPLATE
//
// Some imports not needed every time.
use a_o_c::*;  //import custom lib.rs module
// use std::fs::File;
// use std::error::Error;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
// use std::process;


pub fn day0_challenge1(config: &Config) -> Result<i32, Error> {
    let mut data_struct = DataStruct::new().unwrap();
    perform_calculations(config, &mut data_struct);
    println!("interim caluculations {:?}",data_struct);
    // final calculations below
    Ok(ship_propulsion.gamma*ship_propulsion.epsilon)
}

pub fn day0_challenge2(config: &Config) -> Result<i32, Error> {
    let mut data_struct = DataStruct::new().unwrap();
    perform_calculations(config, &mut data_struct);
    println!("interim caluculations {:?}",data_struct);
    // final calculations below
    Ok(ship_propulsion.gamma*ship_propulsion.epsilon)
}
