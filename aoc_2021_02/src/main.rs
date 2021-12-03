/// Advent of Code 2021 project 02
/// Daniel T. 2021-12-02
/// Part 2 Goal: Find then multiply final horizontal position
/// by the final depth. 

use std::env;
use file_input;

fn main() {
    // Take one commandline argument the path to the file with the 
    // projects input
    let args: Vec<String> = env::args().collect();
    let filename = String::from(&args[1]);
    // for debug, list the input file name. 
    println!("Input filename: {}", filename);

    // Get the contents of the file, pull in from library. 
    let content: Vec<String> = file_input::file_contents_as_vec(filename)
        .expect("Failed to open input file");

    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    for line in content {
        // Honestly this feels a little wasteful in assignments, but oh well
        let command: Vec<&str> = line.split(' ').collect();
        let change: i32 = command[1].parse::<i32>().expect("Failed to parse int command");
        let command: &str = command[0];
        match command {
            "forward" => { position = position + change },
            "down" => { depth = depth + change },
            "up" => { depth = depth - change },
            _other => {}
        }
        
    }

    let position_vector: i32 = position * depth;
    println!("Complete");
    println!("Position: {}, Depth: {}, Final vector: {}", position, depth, position_vector);
}
