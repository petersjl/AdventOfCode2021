use std::fs;

fn main() {
    let contents = fs::read_to_string("sonardepths.txt").expect("Problem reading file");

    let list = contents.split("\n");
    let mut numbers : Vec<i32> = Vec::new();

    for s in list {
        numbers.push(s.parse::<i32>().unwrap());
    }

    let mut sets : Vec<i32> = Vec::new();

    for n in 2..numbers.len(){
        sets.push(numbers[n] + numbers[n - 1] + numbers[n - 2]);
    }

    let mut greater_than = 0;

    for n in 1..sets.len() {
        if sets[n] > sets[n - 1]{
            greater_than = greater_than + 1;
        }
    }
    println!("{}", greater_than);
}