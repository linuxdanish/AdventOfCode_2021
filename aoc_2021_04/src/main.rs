/// Advent of Code 2021 project 04
/// Daniel T. 2021-12-09
/// Part 4.1 Goal: Play bingo with a 5x5 grid and the randomly selected values.
///                 Final value is the sum of all the undrawn numbers times the 
///                 final value that was selected/drawn.
///

use std::env;
use file_input;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;

fn main() {
    // Take one commandline argument, the path to the file with the
    // projects input
    let args: Vec<String> = env::args().collect();
    let filename = String::from(&args[1]);
    // for debug, list the input file name.
    println!("Input filename: {}", filename);

    // get the contents of the file
    let mut content: Vec<String> = file_input::file_contents_as_vec(filename)
        .expect("Failed to open input file");

    // We will need two structures, the list of drawn numbers and the 
    // vector of 5x5 boards. 
    // 3D vector: [boards][rows][column[]
    let mut boards: Vec<Vec<Vec<Number>>> = Vec::new();

    let mut content_iter = content.iter();
    // get the first line of contents, since that will be our 
    // drawn number input
    let drawn_nums = content_iter.next();
    // The rest of the lines will make up our grids
    let mut in_board:bool = true;
    let mut line_cnt = 0;
    let mut brd_ind = 0;
    let mut row_ind = 0;
    let mut clmn_ind = 0;

    // process content into boards
    for line in content_iter {
        // We have to detect when we enter a new boards "data".
        // We could probably detect this just looking for line number, but I'm 
        // instead just look for empty lines.
        if line.len() >= 0 {
            if in_board != true {
                // add a new board (increase index, do I need to push to the vec first?)
                boards.push(vec![]);
                brd_ind += brd_ind;
            }
            // process the line to add to current board
            let numbers:Vec<&str> = line.split(' ').collect();
            // convert numbers into a vector of Number types
            let numbers:Vec<Number> = numbers.iter()
                .map( |x| {
                    let num = Number {
                    value: str::parse::<i32>(x).expect("Failed to parse to int"),
                    marked: false
                    };
                    return num;
                }).collect::<Vec<Number>>();
            // add the row to our grid
            boards[brd_ind].push(numbers);
            in_board = true;
        } else {
            in_board = false;
        }
    }


   // going to need to process the input numbers looking for matches
   // if it is found we will need to mark found. The trick will the 
   // stopping when we find a match.
    // Alright, going to try concurrency with this solution.
    // Our strategy shall be: 
    //  split our boards into six threads (six chosen for pinebook pro)
    //  Create channels for each thread. For each input value we will
    //  send the value to each thread. The thread will check for matches. 
    //  Should a match be found, it will check for bingo. Should bingo be 
    //  found, send result to main thread.
    let concurrency = 6; 
    let mut threads: Vec<ThreadConnection> = Vec::new();
    

    // Break the board vector into groups and create a thread 
    // with each chunk
    
    let board_chunks: Vec<Vec<Vec<Vec<Number>>>>  = boards.chunks(concurrency).map(|c| {
        c.to_owned()}).collect();

    for chunk in board_chunks.into_iter() {
       // Create our communication channels
       let (tx_run, rx_run): (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel();
       let (tx_proc_val, rx_proc_val): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
       let (tx_result, rx_result): (mpsc::Sender<i32>, mpsc::Receiver<i32>) = mpsc::channel();
       // get our chunk
       
       // Spawn thread 
       // this is where we do most of the work here.
        let handle = thread::spawn( move || {
          let mut run = true; 
          let process_chunk = chunk.clone();
          // loop until told to stop processing
          while run {
             let drawn_num = rx_proc_val.recv().expect("unable to recieve processing value");
             // do main process here
             for board in process_chunk.iter() {
                
             }
             run = rx_run.recv().expect("unable to recieve run command");
          }
        });

        threads.push(ThreadConnection{
            handle: handle,
            proc_value: tx_proc_val,
            result: rx_result,
            process: tx_run
        });
    }

}

struct Number {
    value: i32,
    marked: bool
}

impl Clone for Number {
    fn clone(&self) -> Number {
        Number {
            value:  self.value,
            marked: self.marked
        }
    }
}

struct ThreadConnection {
    handle: thread::JoinHandle<()>,
    proc_value: mpsc::Sender<i32>,
    result: mpsc::Receiver<i32>,
    process: mpsc::Sender<bool>
}



