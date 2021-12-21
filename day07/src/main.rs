use std::fs;

fn day7(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let coords: Vec<i32> = data.split(",").map(|num| num.parse::<i32>().unwrap()).collect();

    let mut min: i32 = 420690000;
    let mut min_index = 0;

    let mut min_2: i32 = 420690000;
    let mut min_index_2 = 0;

    for i in 0..*coords.iter().max().unwrap() {
        let sum: i32 = coords.iter().map(|num| (i - num).abs()).sum::<i32>();
        if sum < min {
            min = sum;
            min_index = i;
        }

        let sum_2: i32 = coords.iter().map(|num| { let n = (i - num).abs();
                                                   n * (n + 1) / 2
                                                 }).sum::<i32>();
        if sum_2 < min_2 {
            min_2 = sum_2;
            min_index_2 = i;
        }
    }
    println!("PART1: {} -> {}", min_index, min);
    println!("PART2: {} -> {}", min_index_2, min_2);
}
 

fn main() {
    day7("./input/input.txt");
}
