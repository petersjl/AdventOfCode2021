use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
    let list : Vec<u32> = contents.split(",").flat_map(|x| x.parse::<u32>()).collect();
	let mut sum = 0;
	let mut min = list[0];
	let mut max = list[0];
	for n in &list{
		sum += n;
		if n > &max {max = *n;}
		if n < &min {min = *n;}
	}
	println!("Max: {}, Min: {}, Sum: {}", max, min, sum);
	let pos = (sum / (max - min + 1)) + min;
	let mut fuel = 0;
	for n in &list {
		if n < &pos {fuel += pos - *n} else {fuel += *n - pos}
	}
	println!("Min fuel position: {}\nMin fuel needed: {}", pos, fuel);
	let mut min_fuel = 0;
	let mut min_pos = 0;
	let mut current_fuel;
	for n in &list {
		if n < &min {min_fuel += min - *n} else {min_fuel += *n - min}
	}
	for p in (min + 1)..(max + 1){
		current_fuel = 0;
		for n in &list{
			if n < &p {current_fuel += p - *n} else {current_fuel += *n - p}
		}
		if current_fuel < min_fuel {min_fuel = current_fuel; min_pos = p;}
	}
	println!("n^2 result:\nMin fuel position: {}\nMin fuel needed: {}", min_pos, min_fuel);
}