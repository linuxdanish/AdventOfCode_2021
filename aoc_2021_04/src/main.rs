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
    let content: Vec<String> = file_input::file_contents_as_vec(filename)
        .expect("Failed to open input file");

    // We will need two structures, the list of drawn numbers and the 
    // vector of 5x5 boards. 
    // 3D vector: [boards][rows][column]
    let mut boards: Vec<Vec<Vec<Number>>> = Vec::new();
    boards.push(vec![]);

    let mut content_iter = content.iter();
    // get the first line of contents, since that will be our 
    // drawn number input
    let drawn_nums = content_iter.next();
    //just advance past the first blank line
    content_iter.next();
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
        if !line.is_empty() {
            if in_board != true {
                // add a new board (increase index, do I need to push to the vec first?)
                boards.push(vec![]);
                brd_ind += 1;
            }
            // process the line to add to current board
            let numbers:Vec<&str> = line.split_whitespace().collect();
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
    // count number of boards I found
    println!("Parsed input into {} boards of {} {}", boards.len(), boards[0].len(), boards[0][0].len() );


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
          let mut process_chunk = chunk.clone();
          let mut run = rx_run.recv().expect("unable to recieve initial run command");
          // loop until told to stop processing
          while run {
             let drawn_num = rx_proc_val.recv().expect("unable to recieve processing value");
             // do main process here
             for board in process_chunk.iter_mut() {
                 // we are going to keep track of if each row or column is complete as we go.
                 let mut column_completes = [true, true, true, true, true];
                 let mut row_complete = true;
                for r in 0..board.len() {
                    // reset row_complete on new row
                    row_complete = true;
                    for c in 0..board[r].len() {
                        if drawn_num == board[r][c].value {
                            board[r][c].marked = true;
                        }
                        if board[r][c].marked == false {
                            row_complete = false;
                            column_completes[c] = false;
                        }

                    }
                }
                // Check after updating the board if we completed it
                if row_complete == true || column_completes.contains(&true) {
                    // calculate the result answer.
                    let mut result = 0;
                    // sum all unmarked cells in the board
                    for row in board {
                        for cell in row {
                            if cell.marked == false {
                                result += cell.value;
                            }
                        }
                    }
                    // multiply this result by the process number
                    result = result * drawn_num;
                    // send this to the main thread
                    tx_result.send(result).expect("Failed to send result");
                    // we were successfull, terminate thread
                } else {
                    tx_result.send(-1).expect("Failed to send result code");
                }
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

    // need to conver drawn numbers to i32s
    let drawn_nums:Vec<&str> = drawn_nums.unwrap().split(',').collect();

    let drawn_nums = drawn_nums.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut result = 0;
    '_numbers: for number in drawn_nums {
        for thread in threads.iter() {
            thread.proc_value.send(number).expect("Failed to send thread process number");
            thread.process.send(true).expect("Failed to set the run flag");
        }
        // loop through checking for results from threads
        for thread in threads.iter() {
            let local_result = thread.result.recv().expect("Failed to read result");
            
            match local_result {
                -1 => {},
                i  => result = i,
            };
        }
        if result != 0 {
            // we must have found a result, stop threads and break out of loop
            break '_numbers;
        }
    }

    for thread in threads {
        thread.process.send(false).expect("Failed to stop thread");
        thread.handle.join().expect("Failed to join thread");
    }
    
    println!("Finished, found number: {}", result);
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



