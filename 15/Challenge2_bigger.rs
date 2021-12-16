use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let lines = contents.split("\n");
	let mut map: Vec<Vec<(u8, usize, bool)>> = Vec::new();
	for line in lines{
		let mut row: Vec<(u8, usize, bool)> = Vec::new();
		for c in line.chars(){
			row.push((c.to_digit(10).unwrap() as u8, usize::MAX, false));
		}
		map.push(row);
	}

	let num_rows: usize = map.len();
	let num_cols: usize = map[0].len();
	for row in &mut map {
		for n in 1..5{
			for i in 0..num_cols{
				row.push((((row[i].0 + n - 1) % 9) + 1, usize::MAX, false));
			}
		}
	}
	let num_cols: usize = map[0].len();
	for n in 1..5{
		for r in 0..num_rows{
			let mut row: Vec<(u8, usize, bool)> = Vec::new();
			for c in 0..num_cols{
				row.push((((map[r][c].0 + n - 1) % 9) + 1, usize::MAX, false));
			}
			map.push(row);
		}
	}

	map[0][0].1 = 0;
	let mut visible: HashSet<(usize, usize)> = HashSet::new();
	visible.insert((0,0));
	loop {
		if visible.len() == 0 {break}
		let (r, c): (usize, usize) = find_min(&mut visible, &map);
		map[r][c].2 = true;
		if (r, c) == (map.len() - 1, map[0].len() - 1) {break}
		if r > 0 {
			if !map[r-1][c].2 {
				let dist: usize = map[r][c].1 + map[r-1][c].0 as usize;
				if map[r-1][c].1 > dist {
					map[r-1][c].1 = dist;
					visible.insert((r-1, c));
				}
			}
		}
		if c > 0 {
			if !map[r][c-1].2 {
				let dist: usize = map[r][c].1 + map[r][c-1].0 as usize;
				if map[r][c-1].1 > dist {
					map[r][c-1].1 = dist;
					visible.insert((r, c-1));
				}
			}
		}
		if r < map.len() - 1 {
			if !map[r+1][c].2 {
				let dist: usize = map[r][c].1 + map[r+1][c].0 as usize;
				if map[r+1][c].1 > dist {
					map[r+1][c].1 = dist;
					visible.insert((r+1, c));
				}
			}
		}
		if c < map[0].len() - 1 {
			if !map[r][c+1].2 {
				let dist: usize = map[r][c].1 + map[r][c+1].0 as usize;
				if map[r][c+1].1 > dist {
					map[r][c+1].1 = dist;
					visible.insert((r, c+1));
				}
			}
		}
	}

	println!("Min risk: {}", map[map.len() - 1][map[0].len() - 1].1);
}

fn find_min(visible: &mut HashSet<(usize, usize)>, map: &Vec<Vec<(u8, usize, bool)>>) -> (usize, usize) {
	let (mut r, mut c): (usize, usize) = (map.len() - 1, map[0].len() - 1);
	for (r2, c2) in visible.iter() {
		if map[r][c].1 > map[*r2][*c2].1 {r = *r2; c = *c2;}
	}
	visible.remove(&(r,c));
	return (r, c);
}
