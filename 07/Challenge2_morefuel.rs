use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
    let list : Vec<u32> = contents.split(",").flat_map(|x| x.parse::<u32>()).collect();
	let mut min = list[0];
	let mut max = list[0];
	for n in &list{
		if n > &max {max = *n;}
		if n < &min {min = *n;}
	}
	let mut min_fuel = 0;
	let mut min_pos = 0;
	let mut current_fuel;
	for n in &list {
		let mut step = 1;
			if n < &min {
				for _ in *n..min{
					min_fuel += step;
					step += 1;
				}
			} else {
				for _ in min..*n{
					min_fuel += step;
					step += 1;
				}
			}
	}
	'pos: for p in (min + 1)..(max + 1){
		current_fuel = 0;
		for n in &list{
			let mut step = 1;
			if n < &p {
				for _ in *n..p{
					current_fuel += step;
					if current_fuel > min_fuel {continue 'pos;}
					step += 1;
				}
			} else {
				for _ in p..*n{
					current_fuel += step;
					if current_fuel > min_fuel {continue 'pos;}
					step += 1;
				}
			}
		}
		if current_fuel < min_fuel {min_fuel = current_fuel; min_pos = p;}
	}
	println!("n^3 result:\nMin fuel position: {}\nMin fuel needed: {}", min_pos, min_fuel);
}