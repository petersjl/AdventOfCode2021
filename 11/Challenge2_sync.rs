use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let lines = contents.split("\n");
	let mut map : Vec<Vec<u8>> = Vec::new();
	for line in lines{
		map.push(line.chars().map(|x| {x.to_digit(10).unwrap() as u8}).collect())
	}
	let mut n = 1;
	let size = map.len() * map[0].len();
	'outer: loop {
		for r in &mut map {
			for c in r {
				*c += 1;
			}
		}
		let mut new_flashes;
		let mut step_flashes = 0;
		loop {
			new_flashes = 0;
			for r in 0..map.len(){
				for c in 0..map[0].len(){
					if map[r][c] > 9 {
						new_flashes += 1; 
						flash((r,c), &mut map);
					}
				}
			}
			if new_flashes == 0 {break}
			step_flashes += new_flashes;
		}
		println!("Run {} flashes: {} | {}", n, step_flashes, size);
		if step_flashes == size {break 'outer;}
		n += 1;
	}
	println!("All flashed on run: {}", n);
}

fn flash(point: (usize, usize), map: &mut Vec<Vec<u8>>) {
	map[point.0][ 	point.1] = 0;
	if point.0 > 0 {
		if point.1 > 0 {
			//top left
			if map[point.0 - 1][point.1 - 1] != 0 {map[point.0 - 1][point.1 - 1] += 1};
		}
		//top middle
		if map[point.0 - 1][point.1] != 0 {map[point.0 - 1][point.1] += 1};
		if point.1 < (map[0].len() - 1) {
			//top left
			if map[point.0 - 1][point.1 + 1] != 0 {map[point.0 - 1][point.1 + 1] += 1};
		}
	}
	//middle left
	if point.1 > 0 {
		if map[point.0][point.1 - 1] != 0 {map[point.0][point.1 - 1] += 1};
	}
	//middle right
	if point.1 < (map[0].len() - 1) {
		if map[point.0][point.1 + 1] != 0 {map[point.0][point.1 + 1] += 1};
	}
	if point.0 < (map.len() - 1) {
		if point.1 > 0 {
			//top left
			if map[point.0 + 1][point.1 - 1] != 0 {map[point.0 + 1][point.1 - 1] += 1};
		}
		//top middle
		if map[point.0 + 1][point.1] != 0 {map[point.0 + 1][point.1] += 1};
		if point.1 < (map[0].len() - 1) {
			//top left
			if map[point.0 + 1][point.1 + 1] != 0 {map[point.0 + 1][point.1 + 1] += 1};
		}
	}
}