use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut lines = contents.split(": ");
	let mut coords = lines.nth(1).unwrap().split(", ");
	let mut x_parts = coords.next().unwrap().split("=").nth(1).unwrap().split("..");
	let mut y_parts = coords.next().unwrap().split("=").nth(1).unwrap().split("..");
	let x_min: isize = x_parts.next().unwrap().parse::<isize>().unwrap();
	let x_max: isize = x_parts.next().unwrap().parse::<isize>().unwrap();
	let y_min: isize = y_parts.next().unwrap().parse::<isize>().unwrap();
	let y_max: isize = y_parts.next().unwrap().parse::<isize>().unwrap();

	let mut min_x_velocity: isize = 1;
	let mut test_distance: isize = 1;
	loop {
		if test_distance > x_min {break;}
		min_x_velocity += 1;
		test_distance += min_x_velocity;
	}
	println!("Min x velocity: {}", min_x_velocity);

	let mut shots_hit: isize = 0;
	for y in y_min..=((y_min + 1) * -1){
		for x in min_x_velocity..=x_max{
			let mut x_dist = 0;
			let mut y_dist = 0;
			let mut x_velocity = x;
			let mut y_velocity = y;
			loop {
				x_dist += x_velocity;
				y_dist += y_velocity;
				if x_dist > x_max || y_dist < y_min {break}
				if x_velocity > 0 {x_velocity -= 1}
				y_velocity -= 1;
				if x_dist >= x_min && y_dist <= y_max {
					println!("({},{})", x, y);
					shots_hit += 1;
					break;
				}
			}
		}
	}

	println!("Max hits: {}", shots_hit);
}