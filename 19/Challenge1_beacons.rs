use std::fs;
use std::collections::HashSet;
use std::str::Split;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines: Split<&str> = contents.split("\n\n");
	let mut scanner: Scanner = Scanner::new();
	scanner.add_beacons(&mut lines.next().unwrap().split("\n"));
}

struct Scanner{
	beacons: Vec<Beacon>,
	vectors: HashSet<(i16, i16, i16)>
}

impl Scanner{
	fn new() -> Scanner{
		return Scanner {beacons: Vec::new(), vectors: HashSet::new()}
	}

	fn add_beacons(&mut self, lines: &mut Split<&str>) {
		lines.next();
		for line in lines{
			let mut nums: Split<&str> = line.split(",");
			let x = nums.next().unwrap().parse::<i16>().unwrap();
			let y = nums.next().unwrap().parse::<i16>().unwrap();
			let z = nums.next().unwrap().parse::<i16>().unwrap();
			self.beacons.push(Beacon {x: x, y: y, z: z, vectors: Vec::new()});
		}
		let length = self.beacons.len();
		for n in 0..length{
			for m in 0..length{
				let beacon1 = &mut (self.beacons[n]);
				let beacon2 = &mut (self.beacons[m]);
				let x = beacon1.x - beacon2.x;
				let y = beacon1.y - beacon2.y;
				let z = beacon1.z - beacon2.z;
				let vector = (x, y, z);
				beacon1.vectors.push(vector);
				self.vectors.insert(vector);
			}
		}
	}
}

struct Beacon{
	x: i16,
	y: i16,
	z: i16,
	vectors: Vec<(i16, i16, i16)>
}