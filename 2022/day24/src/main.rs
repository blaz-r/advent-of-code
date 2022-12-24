use std::{fs, collections::{HashSet, VecDeque}};


fn print_grid(grid: &Vec<Vec<char>>){
    for row in grid {
        for e in row {
            print!("{}", e);
        }
        println!();
    }
}


fn move_bliz(grid: &mut Vec<Vec<char>>, bliz: Vec<(usize, usize, i32, i32)>) -> Vec<(usize, usize, i32, i32)> {
    let w = grid[0].len() as i32;
    let h = grid.len() as i32;
    let mut new_bliz = Vec::new();
    for (i, j, di, dj) in bliz {
        let (mut ni, mut nj) = (i as i32 + di, j as i32 + dj); 
        if ni == 0 {
            ni = h - 2;
        }
        if ni == (h - 1) {
            ni = 1;
        }
        if nj == 0 {
            nj = w - 2;
        }
        if nj == (w - 1) {
            nj = 1;
        }
        grid[i][j] = '.';
        new_bliz.push((ni as usize, nj as usize, di, dj))
    }
    for (i, j, _, _) in new_bliz.iter() {
        grid[*i][*j] = '#';
    }

    new_bliz
}


fn solve(mut grid: Vec<Vec<char>>, mut bliz: Vec<(usize, usize, i32, i32)>, i: usize, j: usize, fi: i32, fj: i32) -> (usize, Vec<Vec<char>>, Vec<(usize, usize, i32, i32)>) {
    let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();    
    let mut seen: HashSet<(i32, i32, usize)> = HashSet::new();
    queue.push_back((i, j, 0));

    let mut grids: Vec<Vec<Vec<char>>> = Vec::new();
    let mut blizs: Vec<Vec<(usize, usize, i32, i32)>> = Vec::new();
    for _ in 0..500 {
        grids.push(grid.clone());
        blizs.push(bliz.clone());
        bliz = move_bliz(&mut grid, bliz);
    }
    
    while let Some((ci, cj, time)) = queue.pop_front() {
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)] {
            let (ni, nj) = (ci as i32 + di, cj as i32 + dj);

            if ni == fi && nj == fj {
                return (time, grids[time].clone(), blizs[time].clone());
            }

            if ni < 0 || ni >= grid.len() as i32 || nj < 0 || nj >= grid[0].len() as i32 {
                continue;
            }

            if grids[time][ni as usize][nj as usize] == '.' && !seen.contains(&(ni, nj, time + 1)) {
                seen.insert((ni, nj, time + 1));
                queue.push_back((ni as usize, nj as usize, time + 1));
            }
        }
    }

    (0, grid, bliz)
}


fn day24(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let mut grid: Vec<Vec<char>> = data.lines()
                                   .map(|l| l.chars()
                                                   .collect()
                                       ).collect();

    let mut bliz: Vec<(usize, usize, i32, i32)> = Vec::new();
    for (i, row) in grid.clone().iter().enumerate() {
        for (j, &e) in row.iter().enumerate() {
            match e {
                '>' => bliz.push((i, j, 0, 1)),
                'v' => bliz.push((i, j, 1, 0)),
                '<' => bliz.push((i, j, 0, -1)),
                '^' => bliz.push((i, j, -1, 0)),
                _ => ()
            }
            if e != '.' {
                grid[i][j] = '#';
            }
        }
    }

    let (fi, fj) = (grid.len() as i32 - 1, grid[0].len() as i32 - 2);

    let (t1, g1, b1) = solve(grid, bliz, 0, 1, fi, fj);
    println!("Part 1: {}", t1);
    let (t2, g2, b2) = solve(g1, b1, fi as usize, fj as usize, 0, 1);
    let (tf, _, _) = solve(g2, b2, 0, 1, fi, fj);

    println!("Part 2: {}", t1 + t2 + tf);
}


fn main() {
    day24("input/input.txt");
}
