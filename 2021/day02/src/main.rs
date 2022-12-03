use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn day2<P>(filename: P) 
where P: AsRef<Path>, {
    let lines = read_lines(filename);

    let mut depth_1 = 0;
    let mut horizontal = 0;

    let mut aim = 0;
    let mut depth_2 = 0;
    for line in lines.iter() {
        let split: Vec<&str> = line.split(" ").collect();
        let num: i32 = split[1].parse().unwrap();

        match split[0] {
            "forward" => {
                horizontal += num;      // part 1 & 2
                depth_2 += aim * num},  // part 2
            "down" => {
                depth_1 += num;         // part 1
                aim += num},            // part 2
            "up" => {
                depth_1 -= num;         // part 1
                aim -= num},            // part 2
            &_ => ()
        }
    }

    println!("PART1: {}", depth_1 * horizontal);
    println!("PART2: {}", depth_2 * horizontal);
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
    day2("./input/input.txt")
}
