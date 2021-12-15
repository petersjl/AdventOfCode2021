use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines = contents.split("\n");
	let mut original_polymer : Vec<char> = lines.next().unwrap().chars().collect();
	lines.next();
	let mut rules : HashMap<(char, char), (char, (char, char), (char, char))> = HashMap::new();
	let mut counts : HashMap<char, usize> = HashMap::new();
	for line in lines {
		let mut parts = line.split(" -> ");
		let from : &str = parts.next().unwrap();
		let to : char = parts.next().unwrap().chars().next().unwrap();
		let mut from_chars = from.chars();
		let first : char = from_chars.next().unwrap();
		let second : char = from_chars.next().unwrap();
		rules.insert((first, second), (to, (first, to), (to, second)));
		counts.entry(first).or_insert(0usize);
		counts.entry(second).or_insert(0usize);
		counts.entry(to).or_insert(0usize);
	}
	for s in &original_polymer {
		*counts.get_mut(&s).unwrap() += 1;
	}

	println!("Beginning counts");
	for key in counts.keys() {
		let val = counts.get(key).unwrap();
		println!("{}: {}", key, val);
	}

	let mut current_map : HashMap<(char, char), usize> = HashMap::new();
	for n in 1..original_polymer.len() {
		let entry = current_map.entry((original_polymer[n-1],original_polymer[n])).or_insert(0);
		*entry += 1;
	}
	let runs = 40usize;
	for n in 1..=runs {
		let mut new_map : HashMap<(char, char), usize> = HashMap::new();
		let keys = current_map.keys();
		for key in keys {
			let count : usize = *current_map.get(key).unwrap();
			let to : (char, (char, char), (char, char)) = *rules.get(key).unwrap();
			*counts.get_mut(&to.0).unwrap() += count;
			*new_map.entry(to.1).or_insert(0) += count;
			*new_map.entry(to.2).or_insert(0) += count;
		}
		current_map = new_map;
	}

	let mut keys = counts.keys();
	let key = keys.next().unwrap();
	let val = counts.get(key).unwrap();
	let mut max = val;
	let mut min = val;
	println!("Counts");
	println!("{}: {}", key, val);
	for key in keys {
		let val = counts.get(key).unwrap();
		println!("{}: {}", key, val);
		if val > max {max = val};
		if val < min {min = val};
	}
	println!("Max: {}, Min: {}, Dif: {}", max, min, max - min);
}