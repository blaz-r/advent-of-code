use std::fs;
use std::cmp::{min, max};


fn day5(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let lines = data.lines().filter(|&line| line != "").collect::<Vec<_>>();

    let points: Vec<Vec<&str>> = lines.iter().map(|&line| line.split(" -> ").collect()).collect();

    const SIZE: usize = 1000;
    let mut field = [[0; SIZE]; SIZE];

    for pair in points.iter() {
        let p1: Vec<usize> = pair[0].split(",").map(|num| num.parse().unwrap()).collect();
        let p2: Vec<usize> = pair[1].split(",").map(|num| num.parse().unwrap()).collect();

        if p1[0] == p2[0] {
            for i in min(p1[1], p2[1])..max(p1[1], p2[1]) + 1 {
                field[i as usize][p1[0]] += 1;
            }
        }
        else if p1[1] == p2[1] {
            for i in min(p1[0], p2[0])..max(p1[0], p2[0]) + 1 {
                field[p1[1]][i as usize] += 1;
            }
        }
    }

    let mut twos: usize = field.iter().flatten().filter(|&&num| num >= 2).count();
    println!("PART1: {}", twos);

    for pair in points.iter() {
        let p1: Vec<i32> = pair[0].split(",").map(|num| num.parse().unwrap()).collect();
        let p2: Vec<i32> = pair[1].split(",").map(|num| num.parse().unwrap()).collect();
        
        // diag
        let dx = p2[0] - p1[0];
        let dy = p2[1] - p1[1];

        if dx != 0 && dy != 0 {
            let step_x = dx / dx.abs();
            let step_y = dy / dy.abs();

            for i in 0..dx.abs() + 1 {
                let curr_x = p1[0] + step_x * i;
                let curr_y = p1[1] + step_y * i;
                field[curr_y as usize][curr_x as usize] += 1;
            }
        }
    }
    twos = field.iter().flatten().filter(|&&num| num >= 2).count();
    println!("PART2: {}", twos);
}


fn main() {
    day5("./input/input.txt")
}
