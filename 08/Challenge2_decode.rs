use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines = contents.split("\n");
	let mut lines2 = contents.split("\n");
	let mut noise : Vec<Vec<&str>> = Vec::new();  
	let mut output : Vec<Vec<&str>> = Vec::new(); 
	for next in &mut lines {
		let mut parts = next.split(" | ");
		noise.push(parts.next().unwrap().split_whitespace().collect());
		output.push(parts.next().unwrap().split_whitespace().collect());
	}
	let mut sum = 0;
	for n in 0..noise.len() {
		println!("Decoding line: {}", lines2.next().unwrap());
		sum += decode(&mut noise[n], &mut output[n]);
	}
	println!("Sum: {}", sum);
}

fn decode(noise: &mut Vec<&str>, output: &mut Vec<&str>) -> u32 {
	let letters = ['a','b','c','d','e','f','g'];
	noise.sort_by(|a, b| a.chars().count().cmp(&(b.chars().count())));
	//find a
	let mut a : char = ' ';
	for x in noise[1].chars() {
		if !noise[0].contains(x) {a = x; break;}
	}
	let mut not_in_6lines : String = String::new();
	for line in 6..9 {
		for x in letters {
			if !noise[line].contains(x) {not_in_6lines.push(x); break;}
		}
	}
	let mut c : char = ' ';
	let mut f : char = ' ';
	let one : Vec<char> = noise[0].chars().collect();
	if not_in_6lines.contains(one[0]) {
		c = one[0];
		f = one[1];
	} else {
		c = one[1];
		f = one[0];
	}
	let mut g : char = ' ';
	let mut horizontals = String::new();
	let mut verticals = String::new();
	'letters: for x in letters{
		for n in 3..6 {
			if !noise[n].contains(x) {continue 'letters;}
		}
		horizontals.push(x);
		if horizontals.len() == 3 {break;}
	}
	for x in letters{
		if !horizontals.contains(x) {verticals.push(x);}
	}
	let four : Vec<char> = noise[2].chars().collect();
	let seven : Vec<char> = noise[1].chars().collect();
	for x in horizontals.chars() {
		if four.contains(&x) || seven.contains(&x) {continue;}
		g = x;
		break;
	}
	let mut d : char = ' ';
	for x in horizontals.chars() {
		if !(x == a || x == g) {d = x; break;}
	}
	let mut e : char = ' ';
	let mut three : Vec<char> = Vec::new();
	for x in horizontals.chars() {
		three.push(x);
	}
	three.push(c);
	three.push(f);
	for x in not_in_6lines.chars() {
		if !three.contains(&x) {e = x; break;}
	}
	let mut b : char = ' ';
	for x in letters{
		if !three.contains(&x) && !(x == e) {b = x; break;}
	}
	let zero : Vec<char> = vec![a,b,c,e,f,g];
	let two : Vec<char> = vec![a,c,d,e,g];
	let five : Vec<char> = vec![a,b,d,f,g];
	let six : Vec<char> = vec![a,b,d,e,f,g];
	let eight : Vec<char> = vec![a,b,c,d,e,f,g];
	let nine : Vec<char> = vec![a,b,c,d,f,g];

	let mut number = String::new();
	'thing: for group in output{
		match group.len() {
			2 => {number.push('1')},
			3 => {number.push('7')},
			4 => {number.push('4')},
			7 => {number.push('8')},
			5 => {
				if check_same(group, &two) {number.push('2'); continue 'thing;}
				if check_same(group, &three) {number.push('3'); continue 'thing;}
				if check_same(group, &five) {number.push('5'); continue 'thing;}
			},
			6 => {
				if check_same(group, &zero) {number.push('0'); continue 'thing;}
				if check_same(group, &six) {number.push('6'); continue 'thing;}
				if check_same(group, &nine) {number.push('9'); continue 'thing;}
			},
			_ => println!("Incorrect output {}", group)
		}
	}

	let val = number.parse::<u32>().unwrap();
	println!("Decoded to: {}", val);
	return val
}

fn check_same(group: &str, number: &Vec<char>) -> bool {
	return || -> bool {
		for x in group.chars() {
			if !number.contains(&x) {return false}
		}
		return true;
	}() && || -> bool {
		'outer: for x in number {
			for y in group.chars() {
				if *x == y {continue 'outer}
			}
			return false
		}
		return true;
	}()
}