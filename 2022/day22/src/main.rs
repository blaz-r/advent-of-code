use std::fs;
use regex::Regex;


fn wrap(grid: &Vec<Vec<char>>, ci: usize, cj: usize, ni: i32, nj: i32, di: i32, dj: i32) -> Option<(usize, usize, (i32, i32))> {
    if ni >= grid.len() as i32 || di == 1 && grid[ni as usize][nj as usize] == ' ' {
        for i in 0..grid.len() {
            if grid[i][nj as usize] == '.' {
                return Some((i, nj as usize, (di, dj)));
            }
            if grid[i][cj] == '#' {
                return Some((ci, cj, (di, dj)));
            }
        }
    }

    if ni < 0 || di == -1 && grid[ni as usize][nj as usize] == ' ' {
        for i in (0..grid.len()).rev() {
            if grid[i][cj] == '.' {
                return Some((i, nj as usize, (di, dj)));
            }
            if grid[i][cj] == '#' {
                return Some((ci, cj, (di, dj)));
            }
        }
    }

    if nj >= grid[ci].len() as i32 || dj == 1 && grid[ni as usize][nj as usize] == ' ' {
        for j in 0..grid[ci].len() {
            if grid[ci][j] == '.' {
                return Some((ni as usize, j, (di, dj)));
            }
            if grid[ci][j] == '#' {
                return Some((ci, cj, (di, dj)));
            }
        }
    }

    if nj < 0 || dj == -1 && grid[ni as usize][nj as usize] == ' ' {
        for j in (0..grid[ci].len()).rev() {
            if grid[ci][j] == '.' {
                return Some((ni as usize, j, (di, dj)));
            }
            if grid[ci][j] == '#' {
                return Some((ci, cj, (di, dj)));
            }
        }
    }

    None
}


fn wrap_cube(grid: &Vec<Vec<char>>, i: usize, j: usize, mut ni: i32, mut nj: i32, di: i32, dj: i32) -> (usize, usize, (i32, i32)) {
    let mut ndi = di;
    let mut ndj = dj;

    if ni < 0 && nj >= 50 && nj < 100 && di == -1 {
        ndi = 0;
        ndj = 1;
        ni = nj + 100;
        nj = 0;
    }
    else if nj < 0 && ni >= 150 && ni < 200 && dj == -1 {
        ndi = 1;
        ndj = 0;
        nj = ni - 100;
        ni = 0;
    }
    else if ni < 0 && nj >= 100 && nj < 150 && di == -1 {
        ni = 199;
        nj = nj - 100;
    }
    else if ni >= 200 && nj >= 0 && nj < 50 && di == 1 {
        ni = 0;
        nj = nj + 100;
    }
    else if nj >= 150 && ni >= 0 && ni < 50 && dj == 1 {
        ndj = -1;
        ndi = 0;
        ni = 149 - ni;
        nj = 99;
    }
    else if nj == 100 && ni >= 100 && nj < 150 && dj == 1 {
        ndj = -1;
        ndi = 0;
        ni = 149 - ni;
        nj = 149;
    }
    else if ni == 50 && nj >= 100 && nj < 150 && di == 1 {
        ndi = 0;
        ndj = -1;
        ni = nj - 50;
        nj = 99;
    }
    else if nj == 100 && ni >= 50 && ni < 100 && dj == 1 {
        ndi = -1;
        ndj = 0;
        nj = ni + 50;
        ni = 49;
    }
    else if ni == 150 && nj >= 50 && nj < 100 && di == 1 {
        ndi = 0;
        ndj = -1;
        ni = nj + 100;
        nj = 49;
    }
    else if nj == 50 && ni >= 150 && ni < 200 && dj == 1 {
        ndi = -1;
        ndj = 0;
        nj = ni - 100;
        ni = 149;
    }
    else if ni == 99 && nj >= 0 && nj < 50 && di == -1 {
        ndi = 0;
        ndj = 1;
        ni = nj + 50;
        nj = 50;
    }
    else if nj == 49 && ni >= 50 && ni < 100 && dj == -1 {
        ndi = 1;
        ndj = 0;
        nj = ni - 50;
        ni = 100;
    }
    else if nj == 49 && ni >= 0 && ni < 50 && dj == -1 {
        ndi = 0;
        ndj = 1;
        ni = 149 - ni;
        nj = 0;
    }
    else if nj < 0 && ni >= 100 && ni < 150 && dj == -1 {
        ndi = 0;
        ndj = 1;
        ni = 149 - ni;
        nj = 50;
    }

    if grid[ni as usize][nj as usize] == '#' {
        return (i, j, (di, dj));
    }

    (ni as usize, nj as usize, (ndi, ndj))
}


fn make_move(grid: &Vec<Vec<char>>, i: usize, j: usize, dir: (i32, i32), part1: bool) -> (usize, usize, (i32, i32)) {
    let (di, dj) = dir;
    let (ni, nj) = (i as i32 + di, j as i32 + dj);

    if part1 {
        let wrapped = wrap(grid, i, j, ni, nj, di, dj);

        match wrapped {
            Some(mov) => return mov,
            None => if grid[ni as usize][nj as usize] == '#' {
                return (i, j, dir);
            }
            else {
                return (ni as usize, nj as usize, dir);
            }
        }
    }
    else {
        return wrap_cube(grid, i, j, ni, nj, di, dj);
    }
}


fn password(dists: &Vec<i32>, rots: &Vec<char>, grid: &Vec<Vec<char>>, start: (usize, usize), part1: bool) -> usize {
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let dir_vals = [0, 1, 2, 3];

    let mut dir_id = 0;
    let mut dir = dirs[dir_id];
    let (mut i, mut j) = start;
    for (&dist, &rot) in dists.iter().zip(rots.iter()) {
        for _ in 0..dist {
            (i, j, dir) = make_move(grid, i, j, dir, part1);
        }
        if !part1 {
            dir_id = dirs.iter().position(|d| *d == dir).unwrap();
        }
        match rot {
            'L' => dir_id = (dir_id + 3) % 4, 
            'R' => dir_id = (dir_id + 1) % 4, 
            _ => ()
        }
        dir = dirs[dir_id];
    }

    return (i + 1) * 1000 + (j + 1) * 4 + dir_vals[dir_id];
}


fn day22(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let split: Vec<&str> = data.split("\n\n").collect();
    let width = split[0].lines().map(|l| l.chars().count()).max().unwrap();
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; split[0].lines().count()];
    let num_regex = Regex::new(r"\d+").unwrap();
    let char_regex = Regex::new(r"[A-Z]").unwrap();

    let dists: Vec<i32> = num_regex.find_iter(split[1])
                                   .map(|n| n.as_str()
                                                    .parse::<i32>()
                                                    .unwrap()
                                       ).collect();

    let mut rots: Vec<char> = char_regex.find_iter(split[1])
                                        .map(|n| n.as_str()
                                                         .chars()
                                                         .next()
                                                         .unwrap()
                                            ).collect();

    // since we have 1 less rotation than move, last we just use something else than L and R
    rots.push('s');

    let mut found = false;
    let mut start: (usize, usize) = (0, 0);
    for (i, line) in split[0].lines().enumerate() {
        for (j, e) in line.chars().enumerate() {
            grid[i][j] = e;

            if !found && e == '.' {
                start = (i, j);
                found = true;
            }
        }
    }

    println!("Part 1: {}", password(&dists, &rots, &grid, start, true));
    println!("Part 2: {}", password(&dists, &rots, &grid, start, false));
}


fn main() {
    day22("input/input.txt");
}
