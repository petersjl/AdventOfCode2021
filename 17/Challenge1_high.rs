use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines = contents.split(": ");
	let mut coords = lines.nth(1).unwrap().split(", ");
	let mut x_parts = coords.next().unwrap().split("=").nth(1).unwrap().split("..");
	let mut y_parts = coords.next().unwrap().split("=").nth(1).unwrap().split("..");
	// let x_min: isize = x_parts.next().unwrap().parse::<isize>().unwrap();
	// let x_max: isize = x_parts.next().unwrap().parse::<isize>().unwrap();
	let y_min: isize = y_parts.next().unwrap().parse::<isize>().unwrap();
	// let y_max: isize = y_parts.next().unwrap().parse::<isize>().unwrap();
	let mut y_velocity = (y_min + 1) * -1;
	let mut y_val: isize = 0;
	while y_velocity != 0{
		y_val += y_velocity;
		y_velocity -= 1;
	}
	println!("Max height: {}", y_val);
}