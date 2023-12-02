mod first_part;
mod second_part;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use first_part::first_part_solution;
use second_part::second_part_solution;

fn main() {
    let path = Path::new("input.txt");

    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", path.display(), why);
    }

    let lines: Vec<String> = s.split('\n').map(String::from).collect();
    let lines_2: Vec<String> = s.split('\n').map(String::from).collect();

    let sum_of_ids = first_part_solution(lines);
    let sum_of_ids_2 = second_part_solution(lines_2);
    println!("(first part): Sum of valid game IDs: {}", sum_of_ids);
    println!("(second part): Sum of game IDs: {}", sum_of_ids_2);
}
