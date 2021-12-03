/// Advent of Code 2021 project 02
/// Daniel T. 2021-12-02
/// Part 2.1 Goal: Find then multiply final horizontal position
/// by the final depth. 
/// 
/// Part 2.2 Goal: instructions are different. forward command increases position linearly
/// and increases depth by distance * aim. 
/// up and down control "aim". 
/// Only forward can increase depth.

use std::env;
use file_input;
use std::str;

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

    // closure for parsing a string into a tuple of string + int. 
    // why this rather than a function?
    let parser = | x: &str | ->(String, i32) {
        let split_stuff: Vec<&str> = x.split(' ').collect();
        let command = str::to_owned(split_stuff[0]);
        let mut num: i32 = 0;
        if split_stuff.len() >= 2 {
            num = split_stuff[1].parse::<i32>().unwrap();
        }
        
        return (command, num);
    };

    // create a vector of tuple pairs
    let parsed_content: Vec<(String,i32)> = content.iter().map(|x| 
        parser(x)).collect();

    // part 2.1 loop with it's ugly state.
    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    for command in &parsed_content {        
        match command.0.as_str() {
            "forward" => { position = position + command.1 },
            "down" => { depth = depth + command.1 },
            "up" => { depth = depth - command.1 },
            _other => {}
        }
        
    }
    let position_vector: i32 = position * depth;


    // part 2.2 will use structs.
    let mut current_pos: SubPosition = SubPosition {
        depth: 0,
        aim: 0,
        position: 0,
    };

    for command in &parsed_content {
        match command.0.as_str() {
            "forward" => { 
                current_pos.position = current_pos.position + command.1;
                current_pos.depth = current_pos.depth + (current_pos.aim * command.1);
            },
            "down" => { current_pos.aim = current_pos.aim + command.1 },
            "up" => { current_pos.aim = current_pos.aim - command.1 }
            _other => {}
        }
    }

    let final_pos_vector = current_pos.depth * current_pos.position;
    
    println!("Complete");
    println!("2.1:: Position: {}, Depth: {}, Final vector: {}", position, depth, position_vector);
    println!("2.2:: Position: {}, Depth: {}, Aim: {}, Final vector: {}", current_pos.position, current_pos.depth, current_pos.aim, final_pos_vector)
}

struct SubPosition {
    depth: i32,
    aim: i32,
    position: i32
}