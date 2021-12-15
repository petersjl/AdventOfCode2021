use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines = contents.split("\n");
	let mut current_polymer : Vec<char> = lines.next().unwrap().chars().collect();
	lines.next();
	let mut rules : HashMap<(char, char), char> = HashMap::new();
	let mut counts : HashMap<char, usize> = HashMap::new();
	for line in lines {
		let mut parts = line.split(" -> ");
		let from : &str = parts.next().unwrap();
		let to : char = parts.next().unwrap().chars().next().unwrap();
		let mut from_chars = from.chars();
		let first : char = from_chars.next().unwrap();
		let second : char = from_chars.next().unwrap();
		rules.insert((first, second), to);
		counts.entry(first).or_insert(0usize);
		counts.entry(second).or_insert(0usize);
		counts.entry(to).or_insert(0usize);
	}
	for s in &current_polymer {
		*counts.get_mut(&s).unwrap() += 1;
	}
	
	let keys = counts.keys();
	println!("Beginning counts");
	for key in keys {
		let val = counts.get(key).unwrap();
		println!("{}: {}", key, val);
	}

	let runs = 10;
	for i in 1..=runs {
		let mut new_polymer : Vec<char> = Vec::new();
		new_polymer.push(current_polymer[0]);
		for n in 1..current_polymer.len() {
			let to : char = *rules.get(&(current_polymer[n - 1], current_polymer[n])).unwrap();
			*counts.get_mut(&to).unwrap() += 1;
			new_polymer.push(to);
			new_polymer.push(current_polymer[n]);
		}
		current_polymer = new_polymer;
		// print!("Run {}: ", i);
		// for s in &current_polymer {
		// 	print!("{}", s);
		// }
		// println!("");
		// let mut keys = counts.keys();
		// println!("Counts");
		// for key in keys {
		// 	let val = counts.get(key).unwrap();
		// 	println!("{}: {}", key, val);
		// }
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