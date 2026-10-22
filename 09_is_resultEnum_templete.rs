
use std::fs::read_to_string; // file ko read karne wala function import huaa

fn main() {
    let result = read_to_string("aaa.txt"); // File read karo; result mein Ok(data) ya Err(error) milega.

    match result { //if else hi hai magar better and clean thorugh pattern match
        Ok(data) => println!("{}", data), 
        Err(err) => println!("Error while reading this file: {}", err), // Failure: error ka reason print kiya
    } 
} 
