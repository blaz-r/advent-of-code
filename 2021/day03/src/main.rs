use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;


fn day3<P>(filename:  P)
where P: AsRef<Path> {
    let lines = read_lines(filename);

    let mut oxygen = lines.to_vec();
    let mut co2 = lines.to_vec();

    let mut gamma = "".to_owned();
    let mut epsilon = "".to_owned();

    for i in 0..lines[0].len() {
        // PART 1
        let ones_count = lines.clone()
                              .into_iter()
                              .filter(|line| line.chars().nth(i).unwrap() == '1')
                              .count();

        // most common is 1
        if ones_count > lines.len() - ones_count {
            gamma += "1";
            epsilon += "0";
        }
        // most common is 0
        else {
            gamma += "0";
            epsilon += "1";
        }

        // PART 2
        if oxygen.len() > 1 {
            let oxygen_ones = oxygen.clone()
                                    .into_iter()
                                    .filter(|bits| bits.chars().nth(i).unwrap() == '1')
                                    .count();
            let most_common =  if oxygen_ones >= oxygen.len() - oxygen_ones { '1' } else { '0' };

            oxygen = oxygen.into_iter()
                           .filter(|bits| bits.chars().nth(i).unwrap() == most_common)
                           .collect();
        }
        if co2.len() > 1 {
            let co2_ones = co2.clone()
                              .into_iter()
                              .filter(|bits| bits.chars().nth(i).unwrap() == '1')
                              .count();
            let least_common = if co2_ones >= co2.len() - co2_ones { '0' } else { '1' };

            co2 = co2.into_iter()
                     .filter(|bits| bits.chars().nth(i).unwrap() == least_common)
                     .collect();
        }
    }
    let gamma_num = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon_num = u32::from_str_radix(&epsilon, 2).unwrap();
    let oxygen_num = u32::from_str_radix(&oxygen[0], 2).unwrap();
    let co2_num = u32::from_str_radix(&co2[0], 2).unwrap();

    println!("PART1: {}", gamma_num * epsilon_num);
    println!("PART2: {}", oxygen_num * co2_num);
}


fn read_lines<P>(filename: P) -> Vec<String>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("could not parse line"))
       .collect()
}


fn main() {
    day3("./input/input.txt");
}
