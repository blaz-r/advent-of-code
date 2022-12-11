use std::fs;
use Instr::{Addx, Noop};

#[derive(Debug, Clone, Copy)]
enum Instr {
    Noop,
    Addx(i32)
}


fn print_crt(crt: Vec<Vec<char>>) {
    for row in crt {
        for p in row {
            print!("{p}");
        }
        print!("\n");
    }
}


fn day10(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let instrs: Vec<Instr> = data.lines().map(|line| line.split(" ")
                                                               .collect()
                                             ).into_iter()
                                             .map(|instr: Vec<&str>| {
                                                    if instr.len() == 1 {
                                                        Noop
                                                    }
                                                    else {
                                                        Addx(instr[1].parse().unwrap())
                                                    }
                                                }).collect();

    let mut screen = vec![vec!['.'; 40]; 6];

    let mut sum_1 = 0;

    let mut x = 1;
    let mut cycle = 1;
    let mut i_pos = 0;

    let mut px_pos: usize = 0;
    while i_pos < instrs.len() {
        let instr_len = match instrs[i_pos] {
            Addx(_) => 2,
            Noop => 1 
        };

        for _i in 0..instr_len {
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                sum_1 += x * cycle;
            }

            if i32::abs(px_pos as i32 - x) < 2 {
                screen[((cycle - 1) / 40) as usize][px_pos] = '#'; 
            }

            px_pos += 1;
            px_pos %= 40;

            cycle += 1;
        }

        match instrs[i_pos] {
            Addx(n) => x += n,
            Noop => () 
        };

        i_pos += 1;

    }
    println!("Part 1: {}", sum_1);

    print_crt(screen);
    println!("Part 2: EFUGLPAP");
}


fn main() {
    day10("input/input.txt");
}
