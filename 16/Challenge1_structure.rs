use std::fs;
use std::collections::HashMap;
use std::str::Chars;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Problem reading file");
	let mut binary_map: HashMap<char, &str> = HashMap::new();
	make_binary_map(&mut binary_map);
	let mut binary: String = String::new();
	for c in contents.chars(){
		binary.push_str(binary_map.get(&c).unwrap());
	}
	let mut binary: Vec<char> = binary.chars().collect();
	let version_count = parse_binary_for_version(&binary, 0).1;
	println!("Total version counts: {}", version_count);
}

fn parse_binary_for_version(binary: &Vec<char>, index: usize) -> (usize, usize){ //-> (index, version_sum)
	let mut index = index;
	let mut version_string: String = String::new();
	version_string.push(binary[index]);
	index += 1;
	version_string.push(binary[index]);
	index += 1;
	version_string.push(binary[index]);
	index += 1;
	let version: u8 = u8::from_str_radix(&version_string, 2).unwrap();
	let mut type_string: String = String::new();
	type_string.push(binary[index]);
	index += 1;
	type_string.push(binary[index]);
	index += 1;
	type_string.push(binary[index]);
	index += 1;
	let type_id: u8 = u8::from_str_radix(&type_string, 2).unwrap();
	print!("Version: {}, Type: {}", version, type_id);
	
	match type_id {
		4 => {
			let mut num: String = String::new();
			loop{
				if binary[index].to_digit(2).unwrap() == 1 {
					index += 1;
					num.push(binary[index]);
					index += 1;
					num.push(binary[index]);
					index += 1;
					num.push(binary[index]);
					index += 1;
					num.push(binary[index]);
					index += 1;
				} else {
					index += 1;
					num.push(binary[index]);
					index += 1;
					num.push(binary[index]);
					index += 1;
					num.push(binary[index]);
					index += 1;
					num.push(binary[index]);
					index += 1;
					println!(", Number binary: {}, Number: {}", num, usize::from_str_radix(&num, 2).unwrap());
					return (index, version as usize)
				}
			}
		},
		_ => {
			let mut version_count = version as usize;
			let length_type_id: u32 = binary[index].to_digit(2).unwrap();
			let length: usize = if length_type_id == 1 {11} else {15};
			index += 1;
			// used_bits += 1 + length;
			print!(", Length type: {}", if length_type_id == 1 {"count"} else {"bits"});
			let mut num: String = String::new();
			for _ in 0..length {
				num.push(binary[index]);
				index += 1;
			}
			if length_type_id == 0{
				let body_size: usize = usize::from_str_radix(&num, 2).unwrap();
				println!(", Body size: {}", body_size);
				let max_index = body_size + index;
				loop {
					let (new_index, version_sum) = parse_binary_for_version(binary, index);
					index = new_index;
					version_count += version_sum;
					if index == max_index {break}
				}
				return (index, version_count);
			}else{
				let sub_count: usize = usize::from_str_radix(&num, 2).unwrap();
				println!(", Sub count: {}", sub_count);
				for _ in 0..sub_count {
					let (new_index, version_sum) = parse_binary_for_version(binary, index);
					index = new_index;
					version_count += version_sum;
				}
				return(index, version_count);
			}
		}
	}
}

fn make_binary_map(map: &mut HashMap<char, &str>){
	map.insert('0', "0000");
	map.insert('1', "0001");
	map.insert('2', "0010");
	map.insert('3', "0011");
	map.insert('4', "0100");
	map.insert('5', "0101");
	map.insert('6', "0110");
	map.insert('7', "0111");
	map.insert('8', "1000");
	map.insert('9', "1001");
	map.insert('A', "1010");
	map.insert('B', "1011");
	map.insert('C', "1100");
	map.insert('D', "1101");
	map.insert('E', "1110");
	map.insert('F', "1111");
}