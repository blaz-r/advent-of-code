use std::fs;
use std::cmp::max;


fn day08(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let forest: Vec<Vec<u8>> = data.lines().map(|line| line.bytes()
                                                                                .map(|n| n - ('0' as u8)
                                                                                    ).collect()
                                                                ).collect();
    let mut cand: Vec<(usize, usize)> = Vec::new();
    let mut sum_1 = forest.len() * 2 + (forest[0].len() - 2) * 2;
    for i in 1..(forest.len()-1) {
        for j in 1..(forest[i].len()-1) {
            let el = forest[i][j];
            let up = &forest[..i].iter()
                                         .map(|v| v[j])
                                         .into_iter()
                                         .filter(|&e| e >= el)
                                         .count();
            let down = &forest[(i+1)..].iter()
                                               .map(|v| v[j])
                                               .into_iter()
                                               .filter(|&e| e >= el)
                                               .count();
            let right = &forest[i][(j+1)..].iter()
                                                   .filter(|&&e| e >= el)
                                                   .count();
            let left = &forest[i][..j].iter()
                                              .filter(|&&e| e >= el)
                                              .count();
            if *up == 0  || *down == 0 ||
               *left == 0 || *right == 0 {
                sum_1 += 1;
                cand.push((i , j));
            }
        }
    }
    println!("Part 1: {}", sum_1);

    let mut score_2 = 0;
    for (i, j) in cand {
        let mut curr_score = 1;
        for (di, dj) in [(1, 0), (-1 as i32, 0), (0, 1), (0, -1 as i32)] {

            let mut curr_sum = 0;
            let (mut ci,mut cj) = ((i as i32 + di) as usize, (j as i32 + dj) as usize);
            while ci < forest.len() && cj < forest[i].len() { 
                if forest[ci][cj] < forest[i][j] {
                    curr_sum += 1;
                }
                else {
                    curr_sum += 1;
                    break;
                }
                (ci, cj) = ((ci as i32 + di) as usize, (cj as i32 + dj) as usize);
            }
            curr_score *= curr_sum;
        }
        score_2 = max(curr_score, score_2);
    }
    println!("Part 2: {}", score_2);
}

fn main() {
    day08("input/input.txt");
}
