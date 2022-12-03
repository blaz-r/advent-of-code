use std::fs;


fn day01(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file.");
    let mut elf_cals: Vec<i32> = Vec::new();
    for elf_block in data.split("\n\n") {
        let cals: i32 = elf_block.split("\n").map( |cals| 
                                                    cals.trim()
                                                        .parse::<i32>().unwrap()
                                                 ).sum();
        elf_cals.push(cals);
    }

    // sort descending
    elf_cals.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    let top_1 = elf_cals[0];
    let top_3: i32 = elf_cals[0..3].iter().sum();

    print!("Part 1: {}", top_1);
    print!("Part 2: {}", top_3);
}


fn main() {
    day01("./input/input.txt");
}