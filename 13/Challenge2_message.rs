use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines = contents.split("\n");
	let mut max_x : usize = 0;
	let mut max_y : usize = 0;
	let mut points : Vec<(usize, usize)> = Vec::new();

	loop {
		let next = lines.next().unwrap();
		if next == "" {break}
		let mut parts = next.split(',');
		let x = parts.next().unwrap().parse::<usize>().unwrap();
		let y = parts.next().unwrap().parse::<usize>().unwrap();
		if x > max_x {max_x = x;}
		if y > max_y {max_y = y;}
		points.push((x, y));
	}

	let mut map : Map = Map::new(max_x, max_y);
	map.printsize();
	for point in &points {
		map.mark(point.0, point.1);
	}
	
	loop {
		let next = lines.next();
		if next.is_none() {break}
		let mut fold_instruction_parts = next.unwrap().split(" ").nth(2).unwrap().split("=");
		if fold_instruction_parts.next().unwrap() == "x" {
			map.fold_horizontal(fold_instruction_parts.next().unwrap().parse::<usize>().unwrap());
		}
		else {
			map.fold_vertical(fold_instruction_parts.next().unwrap().parse::<usize>().unwrap());
		}
	}
	println!("Marks: {}", map.count_marks());
	map.print();

}

struct Map {
	map : Vec<Vec<bool>>,
	x: usize,
	y: usize
}

impl Map{
	fn new(x : usize, y : usize) -> Map {
		return Map {x: x + 1, y: y + 1, map : vec![vec![false; x + 1]; y + 1]}
	}

	fn printsize(&self){
		println!("Map with size {}x{}", self.x, self.y);
	}

	fn mark(&mut self, x : usize, y : usize) {
		self.map[y][x] = true;
	}

	fn fold_horizontal(&mut self, x : usize){
		let mut step = 1;
		loop {
			let below = x - step;
			let above = x + step;
			if above == self.x {break}
			for r in 0..self.y {
				self.map[r][below] = self.map[r][below] || self.map[r][above];
			}
			step += 1;
			if step > x {break};
		}
		self.x = x;
		self.printsize();
	}

	fn fold_vertical(&mut self, y : usize){
		let mut step = 1;
		loop {
			let below = y - step;
			let above = y + step;
			if above == self.y {break}
			for c in 0..self.x {
				self.map[below][c] = self.map[below][c] || self.map[above][c];
			}
			step += 1;
			if step > y {break};
		}
		self.y = y;
		self.printsize();
	}

	fn count_marks(&self) -> usize {
		let mut sum : usize = 0;
		for r in 0..self.y{
			for c in 0..self.x{
				if self.map[r][c] {sum += 1;}
			}
		}
		return sum;
	}

	fn print(&self) {
		for r in 0..self.y{
			for c in 0..self.x{
				print!("{}", if self.map[r][c] {'#'} else {'.'})
			}
			println!("");
		}
	}
}