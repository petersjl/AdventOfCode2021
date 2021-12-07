use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
    let list : Vec<u8> = contents.split(",").flat_map(|x| x.parse::<u8>()).collect();
	let mut days = 0;
	let max_days = 256;
	let mut fish : Vec<usize> = vec![0usize; 7];
	let mut new_fish : Vec<usize> = vec![0usize; 7];

	for f in list {
		fish[f as usize % 7] += 1;
	}
	loop {
		new_fish[(days + 2) % 7] = fish[days % 7];
		fish[days % 7] += new_fish[days % 7];
		days += 1;
		if days == max_days {break;}
	}
	let mut sumfish = 0;
	for count in fish {
		sumfish += count;
	}
	sumfish += new_fish[days % 7];
	sumfish += new_fish[(days + 1) % 7];
	println!("In {} days there are {} fish", max_days, sumfish);
}