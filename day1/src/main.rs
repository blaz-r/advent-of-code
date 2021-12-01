use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn day1<P>(filename: P) 
where P: AsRef<Path>, {
    let numbers = read_lines(filename);

    let mut count = 0;

    let mut previous: i32 = numbers[0];
    for num in numbers.iter() {
        if num - previous > 0 {
            count += 1;
        }
        previous = *num;
    }
    
    println!("PART1: {}", count);
    
    let mut window_count = 0;

    let mut prev_window: i32 = numbers[0..3].iter().sum();
    for i in 1..(numbers.len()-2) {
        let curr_window: i32 = numbers[i..(i+3)].iter().sum();
        
        if curr_window - prev_window > 0 {
            window_count += 1;
        }
        prev_window = curr_window;
    }

    println!("PART2: {}", window_count);
}

fn read_lines<P>(filename: P) -> Vec<i32>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("no such file");
    let buf = io::BufReader::new(file);
    buf.lines()
       .map(|l| l.expect("could not parse line").parse().unwrap())
       .collect()
}

fn main() {
    day1("./input/input_day1.txt");
}
