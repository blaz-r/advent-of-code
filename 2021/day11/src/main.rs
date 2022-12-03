use std::fs;


fn flash(octopus: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32{
    if i < 0 || i == octopus.len() as i32 || j < 0 || j == octopus[i as usize].len() as i32 {
        return 0;
    }
    let iu = i as usize;
    let ju = j as usize;

    if octopus[iu][ju] > 0 {
        octopus[iu][ju] += 1;
    }

    let mut flashes = 0;
    let all_dirs = [(0, -1), (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1)];
    if octopus[iu][ju] > 9 {
        flashes += 1;
        octopus[iu][ju] = 0;
        for (di, dj) in all_dirs {
            flashes += flash(octopus, i + di, j + dj);
        }
    }

    flashes
}


fn day11(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let mut octopus: Vec<Vec<i32>> = data.lines()
                                         .map(|line| line.chars()
                                                         .map(|c| c.to_digit(10).unwrap() as i32)
                                                         .collect::<Vec<i32>>()
                                             ).collect();

    const DURATION: i32 = 100;
    let mut num_flashes = 0;
    let mut cycle = 0;
    'outer:  loop {
        octopus = octopus.iter().map(|arr| arr.iter().map(|num| num + 1).collect()).collect();
        for i in 0..octopus.len() {
            for j in 0..octopus[i].len() {
                if octopus[i][j] > 9 {
                    let curr_flashes = flash(&mut octopus, i as i32, j as i32);

                    if cycle < DURATION {
                        num_flashes += curr_flashes;
                    }
                    if curr_flashes == octopus.len() as i32 * octopus[0].len() as i32 {
                        println!("PART2: {}", cycle + 1);

                        break 'outer;
                    }
                }
            }
        }
        cycle += 1;
    }
    println!("PART1: {}", num_flashes);
}


fn main() {
    day11("./input/input.txt");
}
