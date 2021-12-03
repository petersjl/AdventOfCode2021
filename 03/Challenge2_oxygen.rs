use std::fs;

fn main() {
    let contents = fs::read_to_string("readings.txt").expect("Problem reading file");
    let list = contents.split("\n");

    let mut oxygen : Vec<&str> = list.collect();
    let mut scrubber = oxygen.clone();
    //let bitlength : usize = oxygen.get(0).unwrap().len();

    //loop to reduce oxygen vector
    let mut n : usize = 0;
    loop {
        if oxygen.len() == 1 {break;}
        let mut counts = (0,0);
        for line in &oxygen {
            if line.chars().nth(n).unwrap().eq(&'0'){
                counts.0 += 1;
            }else{
                counts.1 +=1;
            }
        }
        let greater = if counts.0 > counts.1 {'0'} else {'1'};
        let mut pos = 0;
        loop {
            if pos >= oxygen.len() {break;}
            if oxygen.get(pos).unwrap().chars().nth(n).unwrap().eq(&greater) {
                oxygen.remove(pos);
            }else{
                pos += 1;
            }
        }
        n += 1;
    }
    n = 0;
    //loop to reduce srcubber size
    loop {
        if scrubber.len() == 1 {break;}
        let mut counts = (0,0);
        for line in &scrubber {
            if line.chars().nth(n).unwrap().eq(&'0'){
                counts.0 += 1;
            }else{
                counts.1 +=1;
            }
        }
        let lesser = if counts.0 <= counts.1 {'0'} else {'1'};
        let mut pos = 0;
        loop {
            if pos >= scrubber.len() {break;}
            if scrubber.get(pos).unwrap().chars().nth(n).unwrap().eq(&lesser) {
                scrubber.remove(pos);
            }else{
                pos += 1;
            }
        }
        n += 1;
    }

    for line in &oxygen{
        println!("{}", line);
    }

    // println!("Oxygen: {}", oxygen);
    // println!("Scrubber: {}", scrubber);
    
    let oxygen_val = i32::from_str_radix(&(oxygen.pop().unwrap().to_string()), 2).unwrap();
    let scrubber_val = i32::from_str_radix(&(scrubber.pop().unwrap().to_string()), 2).unwrap();

    println!("Oxygen is {0}\nScrubber is {1}\nMultiplied is {2}", oxygen_val, scrubber_val, oxygen_val * scrubber_val);
}