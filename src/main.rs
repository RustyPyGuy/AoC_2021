use a_o_c::{Config, run1, run2};
use std::env;
use std::process;

fn main() {
    println!("Advent of Code!");
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_config(&args).unwrap_or_else( |err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let result = match config.challenge {
        1 => run1(config).unwrap(),
        2 => run2(config).unwrap(),
        _ => {println!("Invalid challenge input. Exiting"); process::exit(1);}
    };
    println!("result {}",result);
}
