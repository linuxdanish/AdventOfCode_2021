/// Advent of Code 2021 project 01. 
/// Daniel T. 2021-12-01
/// Part 1 Goal:  How many measurements are larger than the previous messurement?
/// Part 2 Goal: the number of times the sum of measurements in a 3 part sliding
/// window increased. 

use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    // Take command line arguments to get input file
    // We will use simple argument parsing to make this quick
    // Only one argument, input file path
    let args: Vec<String> = env::args().collect();
    let filename = String::from(&args[1]);

    println!("Input filename: {}", filename);

    // Get contents as vector of strings
    let content: Vec<String> = file_contents_as_vec(filename).unwrap();
    // convert contents to ints
    let measurements: Vec<i32> = content.iter().map(|line| {
        line.parse::<i32>().unwrap()
    }).collect::<Vec<i32>>();
    
    let mut increases: i32 = 0;
    let mut decreases: i32 = 0;

    let measurements_iter = measurements.iter();
    // store the first value. The first value always equals itself, and thus 
    // won't count as an increase. There is probably a more "rusty" way to 
    // do this, but I think this will work. It is late. 
    let mut previous: i32 = measurements[0];
    for current in measurements_iter {
        if current > &previous {
            increases = increases + 1;
            println!("{} (Increase)", current);
        }
        else if current < &previous {
            decreases = decreases + 1;
            println!("{} (Decrease)", current);
        }
        else {
            println!("{} (Same)", current);
        }
        previous = *current;
    }

    // start part2 calculations
    let mut prev_window_sum: i32 = 0; // store previous window sum
    let mut window_increases: i32 = 0;
    let mut window_decreases: i32 = 0;
    // sliding window for calculations
    let mut window: std::collections::VecDeque<i32> = std::collections::VecDeque::new();
    // loop through and process values while looping
    let measurements_iter = measurements.iter(); // re-assign to work again
    for current in measurements_iter {
        window.push_back(*current);
        if window.len() == 3 {
            // correct window size calculate sum
            let sum: i32 = window.iter().sum();
            if prev_window_sum == 0 {
                prev_window_sum = sum;
            }
            println!("[{},{},{}] - {}", window[0],window[1],window[2], sum);
            if sum > prev_window_sum {
                window_increases = window_increases + 1;
                println!("Window increase");
            }
            else if sum < prev_window_sum {
                window_decreases = window_decreases + 1;
                println!("window decrease");
            }
            
            prev_window_sum = sum;
            // pop so len goes back to 2 (remove the old value)
            window.pop_front();
        }
    }

    println!("Completed!");
    println!("Increases: {} ; Descreases: {}", increases, decreases);
    println!("Part 2 (sliding window): Increases: {} ; Decreases: {}", window_increases, window_decreases);
}


// Function takes filename and gets the contents as a vector of strings.
fn file_contents_as_vec(filename: String) -> io::Result<Vec<String>> {
    let file = fs::File::open(filename).expect("Failed to open file");
    let reader = io::BufReader::new(file);

    Ok(reader.lines().filter_map(io::Result::ok).collect())
}