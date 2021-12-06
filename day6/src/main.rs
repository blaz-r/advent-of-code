use std::fs;


fn day6(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();

    let mut fish_with_days = vec![0; 9];
    data.split(",").for_each(|num| {let index = num.parse::<usize>().unwrap(); fish_with_days[index] += 1});

    const DURATION: i32 = 256;
    for i in 0..DURATION {
        let first = fish_with_days[0];
        fish_with_days.rotate_left(1);
        fish_with_days[6] += first;

        if i == 79 {
            println!("PART1: {}", fish_with_days.iter().sum::<u64>());
        }
    }
    println!("PART2: {}", fish_with_days.iter().sum::<u64>());
}


fn main() {
    day6("./input/input.txt");
}
