use std::fs;
use std::collections::HashSet;


fn run_sim(steps: &Vec<(&str, i32)>, len: usize) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    // 0->H, all after are tails
    let mut r_pos = vec![(0, 0); len];
    visited.insert(r_pos[0]);

    for (dir, times) in steps {
        for _i in 0..*times {
            let mut h_pos = r_pos[0];
            match *dir {
                "U" => r_pos[0] = (h_pos.0, h_pos.1 + 1),
                "D" => r_pos[0] = (h_pos.0, h_pos.1 - 1),
                "R" => r_pos[0] = (h_pos.0 + 1, h_pos.1),
                "L" => r_pos[0] = (h_pos.0 - 1, h_pos.1),
                _ => panic!("Not possible!!!")
            }

            for r_i in 1..r_pos.len() {
                h_pos = r_pos[r_i-1];
                let t_pos = &mut r_pos[r_i];

                let dx = h_pos.0 - t_pos.0;
                let dy = h_pos.1 - t_pos.1;
                if i32::abs(dx) > 1 &&  i32::abs(dy) > 1 {
                    *t_pos = (t_pos.0 + dx / 2, t_pos.1 + dy / 2);
                }
                else if i32::abs(dy) > 1 {
                    *t_pos = (t_pos.0 + dx, t_pos.1 + dy / 2);
                }
                else if i32::abs(dx) > 1 {
                    *t_pos = (t_pos.0 + dx / 2, t_pos.1 + dy);
                }
            }
            visited.insert(*r_pos.last().unwrap());
        }
    }
    visited.len()
}


fn day09(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let steps: Vec<(&str, i32)> = data.lines()
                    .map(|line| line.split(" ")
                                          .collect::<Vec<&str>>()
                        ).into_iter()
                         .map(|step| (step[0], step[1].parse::<i32>().unwrap()))
                         .collect();

    println!("Part 1: {}", run_sim(&steps, 2));
    println!("Part 2: {}", run_sim(&steps, 10));
}


fn main() {
    day09("input/input.txt");
}
