use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines = contents.split("\n");
	let line = lines.next().unwrap();
	let (mut current_snail, _) = Snail::from(&line.chars().collect(), 0);
	current_snail.print();
	for line in lines {
		current_snail = Snail::from_snails(current_snail, Snail::from(&line.chars().collect(), 0).0);
		current_snail.print();
	}

	println!("Total magnitude is: {}", current_snail.magnitude());

	// println!("Magnitude: {}", Snail::from(&contents.chars().collect(), 0).0.magnitude());
}

struct Snail{
	left: Option<Box<Snail>>,
	right: Option<Box<Snail>>,
	val: u8
}

impl Snail{
	fn from(line: &Vec<char>, index: usize) -> (Snail, usize) {
		let option = line[index].to_digit(10);
		match option {
			Some(x) => {return (Snail {left: None, right: None, val: x as u8}, index + 1)},
			None => {
				let index = index;
				let (left, index) = Snail::from(line, index + 1);
				let (right, index) = Snail::from(line, index + 1);
				return (Snail {left: Some(Box::new(left)), right: Some(Box::new(right)), val: 0}, index + 1)
			}
		}
	}

	fn from_snails(s1: Snail, s2: Snail) -> Snail {
		// print!("Adding: ");
		// s2.print();
		let mut snail = Snail {left: Some(Box::new(s1)), right: Some(Box::new(s2)), val: 0};
		// println!("Added made: ");
		// snail.print();
		loop {
			let mut simple = true;
			loop {
				let exploded = snail.explode(0);
				// snail.print();
				// println!("");
				if !exploded.0 {simple = false;}
				else {break;}
			}
			loop {
				let broken = snail.breakup(0);
				// snail.print();
				// println!("");
				if !broken.0 {simple = false;}
				else {break;}
			}
			if simple {break}
		}
		return snail;
	}

	fn explode(&mut self, depth: u8) -> (bool, i8, i8) {
		if self.left.is_none() {return (true, 0, 0)}
		if depth == 4 {
			let left = self.left.as_ref().unwrap().val;
			let right = self.right.as_ref().unwrap().val;
			self.left = None;
			self.right = None;
			// println!("Exploding {} and {}", left, right);
			return (false, left as i8, right as i8);
		}else {
			let simple = self.left.as_mut().unwrap().explode(depth + 1);
			if !simple.0 {
				if simple.2 != -1 {
					self.right.as_mut().unwrap().add_left(simple.2 as u8);
				}
				return (false, simple.1, -1);
			}
			let simple = self.right.as_mut().unwrap().explode(depth + 1);
			if !simple.0 {
				if simple.1 != -1 {
					self.left.as_mut().unwrap().add_right(simple.1 as u8);
				}
				return (false, -1, simple.2);
			}
		}
		return (true, 0, 0);
	}

	fn breakup(&mut self, depth: u8) -> (bool, i8, i8){
		if self.left.is_some(){
			let simple = self.left.as_mut().unwrap().breakup(depth + 1);
			if !simple.0 {
				if simple.2 != -1 {
					self.right.as_mut().unwrap().add_left(simple.2 as u8);
				}
				return (false, simple.1, -1);
			}
			let simple = self.right.as_mut().unwrap().breakup(depth + 1);
			if !simple.0 {
				if simple.1 != -1 {
					self.left.as_mut().unwrap().add_right(simple.1 as u8);
				}
				return (false, -1, simple.2);
			}
			return (true, 0, 0);
		}
		if self.val > 9 {
			if depth == 4 {
				let val = self.val / 2;
				let rem = self.val % 2;
				// println!("Breaking {} into {} and {}", self.val, val, val + rem);
				self.val = 0;
				return (false, val as i8, (rem + val) as i8);
			}else{
				let val = self.val / 2;
				let rem = self.val % 2;
				// println!("Breaking {} into {} and {}", self.val, val, val + rem);
				self.left = Some(Box::new(Snail {left: None, right: None, val: val}));
				self.right = Some(Box::new(Snail {left: None, right: None, val: val + rem}));
				self.val = 0;
				return(false, -1, -1);
			}
		}else{
			return (true, 0, 0);
		}
	}

	fn add_left(&mut self, val: u8) {
		if self.left.is_none() {
			// println!("adding {} to {}", val, self.val);
			self.val += val
		}
		else {self.left.as_mut().unwrap().add_left(val)}
	}

	fn add_right(&mut self, val: u8) {
		if self.left.is_none() {
			// println!("adding {} to {}", val, self.val);
			self.val += val
		}
		else {self.right.as_mut().unwrap().add_right(val)}
	}

	fn magnitude(&self) -> usize{
		if self.left.is_none(){
			return self.val as usize;
		}
		else{
			return 3 * self.left.as_ref().unwrap().magnitude() + 2 * self.right.as_deref().unwrap().magnitude();
		}
	}

	fn print(&self) {
		if self.left.is_none() {
			print!("{}", self.val);
		}else{
			print!("[");
			self.left.as_ref().unwrap().print_inner();
			print!(",");
			self.right.as_ref().unwrap().print_inner();
			println!("]");
		}
	}

	fn print_inner(&self){
		if self.left.is_none() {
			print!("{}", self.val);
		}else{
			print!("[");
			self.left.as_ref().unwrap().print_inner();
			print!(",");
			self.right.as_ref().unwrap().print_inner();
			print!("]");
		}
	}
}