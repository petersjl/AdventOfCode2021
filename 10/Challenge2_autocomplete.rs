use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let lines = contents.split("\n");
	let starts = ['[','{','(','<'];
	let mut scores : Vec<u64> = Vec::new();
	'lines: for line in lines {
		let chars : Vec<char> = line.chars().collect();
		let mut open : Vec<char> = Vec::new();
		// println!("Checking line: {}", line);
		for n in 0..chars.len() {
			let c = chars[n];
			if starts.contains(&c) { open.push(c) }
			else {
				if open.len() == 0 {break;}
				let x = open.pop().unwrap();
				// println!("x: {}, c: {}", x, c);
				match c {
					')' => {if x != '(' {continue 'lines}},
					']' => {if x != '[' {continue 'lines}},
					'}' => {if x != '{' {continue 'lines}}
					'>' => {if x != '<' {continue 'lines}},
					_ => println!("Invalid character")
				}				
			}
		}
		let mut line_score : u64 = 0;
		loop{
			if open.len() == 0 {break;}
			let c = open.pop().unwrap();
			line_score *= 5;
			line_score += match c {
				'(' => 1,
				'[' => 2,
				'{' => 3,
				'<' => 4,
				_ => {println!("Invalid character"); 0}
			}
		}
		scores.push(line_score);
	}
	scores.sort();
	for score in &scores{
		println!("{}", score);
	}
	println!("Score: {}", scores[scores.len()/2]);
}