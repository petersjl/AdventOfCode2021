use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let lines = contents.split("\n");
	let mut map : Vec<Vec<u8>> = Vec::new();
	for line in lines {
		map.push(line.chars().map(|x| {x.to_digit(10).unwrap() as u8}).collect());
	}
	let mut lows : Vec<(u32,u32)> = Vec::new();
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
			lows.push((r as u32,c as u32));
		}
	}
	let mut total_size = 1;
	let mut low_sizes : Vec<u32> = vec![0u32;3];
	for low in lows {
		// println!("\nChecking low ({},{})", low.0, low.1);
		let mut open : Vec<(u32,u32)> = Vec::new();
		let mut closed : Vec<(u32,u32)> = Vec::new();
		open.push((low.0,low.1));
		let mut size = 1;
		while open.len() != 0 {
			let current : (u32,u32) = open.remove(0);
			// println!("Looking around point ({},{})", current.0, current.1);
			if current.0 > 0 {
				let point = (current.0 - 1, current.1);
				// print!("Checking up: ");
				if !(map[point.0 as usize][point.1 as usize] == 9) {
					if !(open.iter().any(|p| {p.0 == point.0 && p.1 == point.1}) || closed.iter().any(|p| {p.0 == point.0 && p.1 == point.1})) {
						// println!("+");
						open.push(point);
						size += 1;
					} 
					// else {
					// 	println!("");
					// }
				}
				// else {
				// 	println!("9");
				// }
			}
			if current.0 < (map.len() as u32) - 1 {
				let point = (current.0 + 1, current.1);
				// print!("Checking down: ");
				if !(map[point.0 as usize][point.1 as usize] == 9) {
					if !(open.iter().any(|p| {p.0 == point.0 && p.1 == point.1}) || closed.iter().any(|p| {p.0 == point.0 && p.1 == point.1})) {
						// println!("+");
						open.push(point);
						size += 1;
					}
					// else {
					// 	println!("");
					// }
				}
				// else {
				// 	println!("9");
				// }
			}
			if current.1 > 0 {
				let point = (current.0, current.1 - 1);
				// print!("Checking left: ");
				if !(map[point.0 as usize][point.1 as usize] == 9) {
					if !(open.iter().any(|p| {p.0 == point.0 && p.1 == point.1}) || closed.iter().any(|p| {p.0 == point.0 && p.1 == point.1})) {
						// println!("+");
						open.push(point);
						size += 1;
					}
					// else {
					// 	println!("");
					// }
				}
				// else {
				// 	println!("9");
				// }
			}
			if current.1 < (map[0].len() as u32) - 1 {
				let point = (current.0, current.1 + 1);
				// print!("Checking right: ");
				if !(map[point.0 as usize][point.1 as usize] == 9) {
					if !(open.iter().any(|p| {p.0 == point.0 && p.1 == point.1}) || closed.iter().any(|p| {p.0 == point.0 && p.1 == point.1})) {
						// println!("+");
						open.push(point);
						size += 1;
					}
					// else {
					// 	println!("");
					// }
				}
				// else {
				// 	println!("9");
				// }
			}
			closed.push(current);
		}
		println!("Size found {}", size);
		for n in 0..low_sizes.len() {
			if low_sizes[n] < size {
				low_sizes.insert(n, size);
				low_sizes.remove(3);
				break;
			}
		}
		print!("Current sizes: ");
		for size in &low_sizes{
			print!("{} ", size);
		}	
		println!("");
	}
	for size in low_sizes{
		total_size *= size;
	}

	println!("Total size: {}", total_size);
}