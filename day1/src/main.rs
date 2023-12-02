mod first_part;
mod second_part;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::first_part::{concat_inputs, first_part_solution};
use crate::second_part::{concat_numbers, second_part_solution};


fn main() {
    let path = Path::new("input.txt");

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}", why),
        Ok(file) => file,
    };
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) { panic!("couldn't read {}: {}", path.display(), why) }

    let lines: Vec<String> = s.split('\n').map(|line| line.to_string()).collect();
    println!("Solution first part: {:?}", first_part_solution(concat_inputs(lines)));
    let lines2: Vec<String> = s.split('\n').map(|line| line.to_string()).collect();
    println!("Solution second part: {:?}", second_part_solution(concat_numbers(lines2)));
}
