use std::fs;
use std::collections::HashSet;

fn day04(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");

    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for line in data.lines() {
        let sections: Vec<Vec<i32>> = line.split(",")
                                          .map(|sect| 
                                                    sect.split("-")
                                                        .map(|num| 
                                                                num.parse::<i32>()
                                                                   .unwrap()
                                                            ).collect()
                                              ).collect();

        let first: HashSet<i32> = HashSet::from_iter(sections[0][0]..=sections[0][1]);
        let second: HashSet<i32> = HashSet::from_iter(sections[1][0]..=sections[1][1]);
        let common = first.intersection(&second);

        if common.clone().count() == first.len() || common.clone().count() == second.len() {
            sum_1 += 1;
        }
        if common.count() > 0 {
            sum_2 += 1;
        }
    }
    
    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);
}


fn main() {
    day04("input/input.txt");
}
