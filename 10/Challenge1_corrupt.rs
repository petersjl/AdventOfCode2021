use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let lines = contents.split("\n");
	let starts = ['[','{','(','<'];
	let ends = [']','}',')','>'];
	let mut score = 0;
	for line in lines {
		let mut chars : Vec<char> = line.chars().collect();
		let mut open : Vec<char> = Vec::new();
		// println!("Checking line: {}", line);
		'chars: for n in 0..chars.len() {
			let c = chars[n];
			if starts.contains(&c) { open.push(c) }
			else {
				if open.len() == 0 {break;}
				let x = open.pop().unwrap();
				// println!("x: {}, c: {}", x, c);
				match c {
					')' => {if x != '(' {score += 3; println!("Found illegal: ) in line \"{}\" at index {}" , line, n); break 'chars;}},
					']' => {if x != '[' {score += 57; println!("Found illegal: ] in line \"{}\" at index {}" , line, n); break 'chars;}},
					'}' => {if x != '{' {score += 1197; println!("Found illegal: }} in line \"{}\" at index {}" , line, n); break 'chars;}}
					'>' => {if x != '<' {score += 25137; println!("Found illegal: > in line \"{}\" at index {}" , line, n); break 'chars;}},
					_ => println!("Invalid character")
				}				
			}
		}
	}
	println!("Score: {}", score);
}