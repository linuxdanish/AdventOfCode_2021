/// Advent of Code 2021 project 03
/// Daniel T. 2021-12-04
/// Part 3.1 Goal: Find the total power usage of our submarine.
///                Power = E * G. E
///                Gamma is the most common bit in each position of
///                all input numbers
///                Epsilon is the least common bit value in each position
///                of all inpt numbers
/// Part3.2 Goal: Find a singular number as the result of a closing mask window
///               the criteria for each successive progression of the window will
///               be: Oxygen (most common, rounding to 1); Co2 (least common rounding to 0)
///

use std::env;
use file_input;
use b_tree_lib;


fn main() {
    // Take one commandline argument, the path to the file with the 
    // projects input
    let args: Vec<String> = env::args().collect();
    let filename = String::from(&args[1]);
    // for debug, list the input file name.
    println!("Inpt fillename: {}", filename);

    // get the contents of the file
    let content: Vec<String> = file_input::file_contents_as_vec(filename)
        .expect("Failed to open input file");

    // need to build the structures to keep 
    let mut position_cnts: Vec<ValueCnt> = vec![ValueCnt::default(); 12];
    
    // process all our file content and count characters. 
    for line in content.iter() {
        // Our input is the same length, but figured I should check.
        if line.len() <= position_cnts.len() {
            for character in line.char_indices() {
                match character.1 {
                    '1' => { position_cnts[character.0].ones = position_cnts[character.0].ones + 1 },
                    '0' => { position_cnts[character.0].zeros = position_cnts[character.0].zeros + 1 },
                    _ => {}
                }
            }
        }
    }
    
    // build the final gamma and episilon strings/values
    let gamma = position_cnts.iter().map( |x| x.largest_value() ).collect::<String>();
    let epsilon = position_cnts.iter().map( |x| x.smallest_value()).collect::<String>();

    // Convert the strings to values
    let gamma = i32::from_str_radix(&gamma,2).expect("Failed to convert gamma to int");
    let epsilon = i32::from_str_radix(&epsilon,2).expect("Failed to convert epsilon gto int");

    // final power for part 3.1
    let power = gamma * epsilon;

    // Part3.2 going to just re-perform the tree building here as a seperate part 
    // of this solution so as to not muddy part 1.
    let mut tree_head: Box<b_tree_lib::Node<char>> = Box::new(b_tree_lib::Node::new('+'));
    for line in content.iter() {
        // reset cur_head to the head of our tree
        let mut cur_head = &mut tree_head;
        for character in line.chars() {
            match character { // decide whch path to take '0' = l, '1' = r
                '1' => {
                    // check if next node already exists, if not, then wee have to add it.
                    match &cur_head.r {
                        Some(_x) => {
                            // cur_head.r exists, we move into it and increase it's visit count
                            cur_head = cur_head.r.as_mut().unwrap();
                            cur_head.count = cur_head.count + 1;
                        },
                        None => {
                            let new_node: Box<b_tree_lib::Node<char>> = Box::new(b_tree_lib::Node::new('1'));
                            cur_head.r = Some(new_node);
                            cur_head = cur_head.r.as_mut().unwrap();
                        }
                    }
                },
                '0' => {
                    // repeat for the left node if is a 0
                    match &cur_head.l {
                        Some(_x) => {
                            // cur_head.l exists, we move into it an increase it's visit count
                            cur_head = cur_head.l.as_mut().unwrap();
                            cur_head.count = cur_head.count +1;
                        },
                        None => {
                            let new_node: Box<b_tree_lib::Node<char>> = Box::new(b_tree_lib::Node::new('0'));
                            cur_head.l = Some(new_node);
                            cur_head = cur_head.l.as_mut().unwrap();
                        }
                    }
                },
                _ => {}
            }
        }
    }

    // We now need to perform out two searches across the binary tree. 
    let oxy_selector = |zeros, ones| { 
        if ones >= zeros {
            1
        } else {
            0
        }
    };

    let oxy_value_str = retrieve_tree_path(&mut tree_head, oxy_selector);

    let co2_selector = |zeros, ones| {
        if zeros <= ones {
            if zeros > 0 {
                0
            } else {
                1
            }
        }else {
            if ones > 0 {
                1
            }else {
                0
            }
        }
    };

    let co2_value_str = retrieve_tree_path(&mut tree_head, co2_selector);
    // convert my strings to decimal from base 2 and get final values
    let oxy_value = i32::from_str_radix(&oxy_value_str,2).expect("Failed to convert oxygen value to int");
    let co2_value = i32::from_str_radix(&co2_value_str,2).expect("Failed to convert co2 value to int");
    let life_support = oxy_value * co2_value;

    println!("Complete!");
    println!("Part 3.1: Gamma: {}, Epsilon {}, Power: {}",gamma,epsilon,power);
    println!("Part 3.2: Oxygen gen rating: {}; co2 scrubber rating: {}; Life support rating {}", oxy_value,co2_value,life_support);
        
}

fn retrieve_tree_path<F: Fn(i32,i32)->i32>(tree: &Box<b_tree_lib::Node<char>>, selector: F) -> String {
    let mut cur_head = tree;
    let mut chars: Vec<char> = Vec::new();
    // walk the tree until we hit the bottom
    while cur_head.l.as_ref().is_some() || cur_head.r.as_ref().is_some() {
        // retrieve left and right counts of sub nodes
        let lcount = if cur_head.l.is_some() {
            cur_head.l.as_ref().expect("Failed to unwrap cur_head.l count").count
        } else {
            0
        };

        let rcount = if cur_head.r.is_some() {
            cur_head.r.as_ref().expect("Failed to unwrap cur_head.r count").count
        } else {
            0
        };
        
        // Decide which way to traverse. We will go left on '0' right on '1'
        if selector(lcount, rcount) == 1 {
            cur_head = cur_head.r.as_ref().expect("Failed to unwrap cur_head.r");
        } else {
            cur_head = cur_head.l.as_ref().expect("Failed to unwrap cur_head.l");
        };
        // extract the new char value and add it to our vector.cur_head
        chars.push(*cur_head.value);
    }

    return chars.into_iter().collect::<String>();
}

#[allow(unused)]
#[derive(Clone)]
struct ValueCnt {
    ones: i32,
    zeros: i32
}

// Try methods. 
impl Default for ValueCnt {
    fn default() -> Self {
        Self { ones: 0, zeros: 0 }
    }
}

impl ValueCnt {
    fn largest_value(&self) -> char {
        match self.ones > self.zeros {
             true => '1',
            false => '0'
        }
    }

    fn smallest_value(&self) -> char {
        match self.ones < self.zeros {
            true => '1',
            false => '0'
        }
    }
}
