use std::fs;
use std::collections::HashSet;


fn day03(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let rucksacks: Vec<Vec<u8>> = data.lines()
                                        .map(|line|
                                                line.bytes()
                                                    .collect()
                                            ).collect();

    let mut sum_1: i32 = 0;
    let mut sum_2: i32 = 0;
    for rucksack in rucksacks.clone().iter() {
        let mid = rucksack.len() / 2;
        let left: HashSet<u8> = HashSet::from_iter(rucksack[..mid].iter().cloned());
        let right: HashSet<u8> = HashSet::from_iter(rucksack[mid..].iter().cloned());    

        let common = left.intersection(&right);
        
        for item in common {
            let priority = 
            if *item >= 97 {
                item - 96
            }
            else {
                item - 65 + 27
            };

            sum_1 += priority as i32;
        }
    }

    for i in (0..rucksacks.len()).step_by(3) {
        let first: HashSet<u8> = HashSet::from_iter(rucksacks[i].iter().cloned());
        let second: HashSet<u8> = HashSet::from_iter(rucksacks[i + 1].iter().cloned());    
        let third: HashSet<u8> = HashSet::from_iter(rucksacks[i + 2].iter().cloned());    

        let common_2: HashSet<u8> = first.intersection(&second).copied().collect();
        let common = common_2.intersection(&third);
        
        for item in common {
            let priority = 
            if *item >= 97 {
                item - 96
            }
            else {
                item - 65 + 27
            };

            sum_2 += priority as i32;
        }
    }

    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);
}


fn main() {
    day03("./input/input.txt");
}
