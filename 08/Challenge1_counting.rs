use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines = contents.split("\n");
	let mut output : Vec<&str> = Vec::new(); 
	loop {
		let next = lines.next();
		if next.is_none() {break;}
		let mut parts = next.unwrap().split(" | ");
		parts.next();
		output.append(&mut parts.next().unwrap().split_whitespace().collect());
	}
	let mut count = 0;
	for line in output {
		let len = line.chars().count();
		if [2,3,4,7].contains(&len) {count += 1;}
	}
	println!("Appearances: {}", count);
}