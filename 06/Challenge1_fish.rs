use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
    let mut fish : Vec<u8> = contents.split(",").flat_map(|x| x.parse::<u8>()).collect();
	let mut days = 0;
	let max_days = 80;
	loop {
		let length = fish.len();
		for n in 0..length {
			if fish[n] == 0{
				fish[n] = 6;
				fish.push(8u8);
			}else {
				fish[n] -= 1;
			}
		}
		days += 1;
		if days == max_days {break;}
	}
	println!("In {} days there are {} fish", max_days, fish.len());
}