use std::fs;

fn main() {
    let contents = fs::read_to_string("readings.txt").expect("Problem reading file");
    let list = contents.split("\n");

    let strings : Vec<&str> = list.collect();
    let bitlength : usize = strings.get(0).unwrap().len();
    let mut counts = vec![(0,0);bitlength];

    for line in strings {
        let mut bits = line.chars();
        for n in 0..bitlength {
            let bit : char = bits.next().unwrap();
            if bit.eq(&'0') {
                counts[n].0 += 1;
            }else{
                counts[n].1 += 1;
            }
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for n in 0..bitlength {
        let t = counts.get(n).unwrap();
        println!("Counts for bit {0}: ({1},{2})", n, t.0,t.1);
        if t.0 > t.1 {
            gamma.push('0');
            epsilon.push('1');
        }else{
            gamma.push('1');
            epsilon.push('0');
        }
    }

    println!("Gamma: {0}\nEpsilon: {1}", gamma, epsilon);

    let gamma_num = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_num = i32::from_str_radix(&epsilon, 2).unwrap();

    println!("Gamma is {0}\nEpsilon is {1}\nMultiplied is {2}", gamma_num, epsilon_num, gamma_num * epsilon_num);
}