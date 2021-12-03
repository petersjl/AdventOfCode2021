use std::fs;

fn main() {
    let contents = fs::read_to_string("directions.txt").expect("Problem reading file");

    let list = contents.split("\n");
    
    let mut distance = 0;
    let mut height :i32 = 0;

    for s in list {
        let mut direction = s.split(" ");
        let command : String = direction.next().unwrap().to_string();
        let count : i32 = direction.next().unwrap().parse().unwrap();

        if command.eq("forward") {
            distance = distance + count;
        }
        else if command.eq("up") {
            height = height - count;
        }
        else if command.eq("down") {
            height = height + count;
        }else {
            println!("Encountered unexpected token \"{}\"", command);
        }
    }

    println!("Direction is {0}\nHeight is {1}\nMultiplied is {2}", distance, height, distance * height);
}