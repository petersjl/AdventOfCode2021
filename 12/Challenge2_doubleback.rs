use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let lines = contents.split("\n");
	let mut passages : HashMap<&str, HashSet<&str>> = HashMap::new();
	for line in lines {
		let positions : Vec<&str> = line.split("-").collect();
		let first = (&mut passages).entry(positions[0]).or_insert(HashSet::new());
		first.insert(positions[1]);
		let second = (&mut passages).entry(positions[1]).or_insert(HashSet::new());
		second.insert(positions[0]);
	}
	let mut routes : HashSet<String> = HashSet::new();
	let mut open : VecDeque<(String, &str, bool)> = VecDeque::new();
	open.push_back((String::from("start"), "start", false));
	loop {
		if open.len() == 0 {break}
		let current : (String, &str, bool) = open.pop_front().unwrap();
		let destinations : &HashSet<&str> = passages.get(&current.1).unwrap();
		for d in destinations {
			if d == &"start" {continue}
			let mut new_path : String = current.0.clone();
			new_path.push_str(d);
			if d == &"end" {routes.insert(new_path); continue}
			if d.chars().any(|x| x.is_lowercase()){
				if current.0.contains(d) {
					if !current.2 {open.push_back((new_path, d, true))}
				}else {
					open.push_back((new_path, d, current.2))
				}
			}
			else {
				open.push_back((new_path, d, current.2))
			}
		}
	}
	// for route in &routes {
	// 	println!("{}", route);
	// }
	println!("Number of routes: {}", routes.len());
}