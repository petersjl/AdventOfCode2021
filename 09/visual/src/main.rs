use std::fs;
use colored::*;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let lines = contents.split("\n");
	let mut map : Vec<Vec<u8>> = Vec::new();
	for line in lines {
		map.push(line.chars().map(|x| {x.to_digit(10).unwrap() as u8}).collect());
	}
    for r in map {
        for c in r {
            if c == 9 {print!("{}", c.to_string().red())}
            else {print!("{}", c)}
        }
        println!("");
    }
}