use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let lines = contents.split("\n");
	let mut map : Vec<Vec<u8>> = Vec::new();
	for line in lines {
		map.push(line.chars().map(|x| {x.to_digit(10).unwrap() as u8}).collect());
	}
	let mut risk = 0u32;
	for r in 0..map.len(){
		for c in 0..map[0].len(){
			let current = map[r][c];
			if r > 0 {
				if map[r - 1][c] <= current {continue}
			}
			if r < map.len() - 1 {
				if map[r + 1][c] <= current {continue}
			}
			if c > 0 {
				if map[r][c - 1] <= current {continue}
			}
			if c < map[0].len() - 1 {
				if map[r][c + 1] <= current {continue}
			}
			risk += (map[r][c] as u32) + 1;
		}
	}
	println!("Total risk: {}", risk);
}