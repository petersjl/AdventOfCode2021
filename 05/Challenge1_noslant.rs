use std::fs;

fn main() {
	//read contents and set up line holder
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
    let list = contents.split("\n");
	let mut lines : Vec<Line> = Vec::new(); 

	//parse lines and find the max x and max y
	let mut max_x : u16 = 0;
	let mut max_y : u16 = 0;
	for l in list {
		let mut iter = l.split(" -> ");
		let mut start_vals = iter.next().unwrap().split(",");
		let mut end_vals = iter.next().unwrap().split(",");
		let point1 = Point {x: start_vals.next().unwrap().parse::<u16>().unwrap(), y: start_vals.next().unwrap().parse::<u16>().unwrap()};
		let point2 = Point {x: end_vals.next().unwrap().parse::<u16>().unwrap(), y: end_vals.next().unwrap().parse::<u16>().unwrap()};
		if point1.x > max_x {max_x = point1.x};
		if point2.x > max_x {max_x = point2.x};
		if point1.y > max_y {max_y = point1.y};
		if point2.y > max_y {max_y = point2.y};
		lines.push(Line {start: point1, end: point2});
	}
	let mut map = build_map(max_x, max_y);

	let mut straight_lines : Vec<Line> = Vec::new();
	for line in lines {
		if line.start.x == line.end.x || line.start.y == line.end.y {straight_lines.push(line)};
	}

	for line in straight_lines {
		if line.start.x == line.end.x {
			let step : i16 = if line.start.y < line.end.y {1} else {-1};
			let mut current = line.start.y as i16;
			let end = line.end.y as i16 + step;
			while current != end {
				map.mark(line.start.x as usize, current as usize);
				current += step;
			}
		}else {
			let step : i16 = if line.start.x < line.end.x {1} else {-1};
			let mut current = line.start.x as i16;
			let end = line.end.x as i16 + step;
			while current != end {
				map.mark(current as usize, line.start.y as usize);
				current += step;
			}
		}
	}
	println!("Spots greater than 1: {}", map.count_greater_than(1));

}

struct Point {
	x: u16,
	y:u16
}

struct Line {
	start: Point,
	end: Point
}

impl Line {
	// fn print(&self) {
	// 	println!("({}, {}) -> ({}, {})", self.start.x,self.start.y,self.end.x,self.end.y);
	// }
}

struct Map {
	map: Vec<Vec<u32>>
}

impl Map {
	fn mark(&mut self, x:usize, y:usize) -> u32 {
		let num = self.map[y][x] + 1;
		self.map[y][x] = num;
		num
	}
	fn count_greater_than(&self, num: u32) -> usize{
		let mut count = 0usize;
		for r in 0..self.map.len(){
			for c in 0..self.map[0].len(){
				if self.map[r][c] > num {count += 1;}
			}
		}
		count
	}
	// fn print(&self){
	// 	for r in 0..self.map.len(){
	// 		for c in 0..self.map[0].len(){
	// 			print!("{} ", if self.map[r][c] == 0 {String::from("x")} else {self.map[r][c].to_string()});
	// 		}
	// 		println!("");
	// 	}
	// }
}

fn build_map(x: u16, y: u16) -> Map {
	let mut vec : Vec<Vec<u32>> = Vec::new();
	let inner : Vec<u32> = vec![0u32; (x + 1) as usize];
	for _ in 0..(y + 1) {
		vec.push(inner.clone());
	}
	Map {map: vec}
}