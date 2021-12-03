use std::fs;

fn main() {
    let contents = fs::read_to_string("sonardepths.txt").expect("Problem reading file");

    let list = contents.split("\n");

    let mut greater_than = 0;
    let mut previous: i32 = -1;

    for s in list {
        if !(previous == -1){
            let current: i32 = s.parse().unwrap();

            if current > previous {
                greater_than = greater_than + 1;
            }
            previous = current;
        }else{
            previous = s.parse().unwrap();
            println!("First value: {}", previous)
        }
    }
    println!("{}", greater_than);
}