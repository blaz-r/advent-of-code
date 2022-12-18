use std::fs;

/*
    ADVENT OF FOR LOOPS :DDDDDD
*/


fn day18(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let coords: Vec<Vec<usize>> = data.lines()
                                     .map(|line| line.split(",")
                                                           .map(|n| n.parse().unwrap()).collect()
                                         ).collect();
    let mut grid = [[[0; 40]; 40]; 40];
    for cube in coords {
        let (x, y, z) = (cube[0] + 5, cube[1] + 5, cube[2] + 5);
        grid[x][y][z] = 1;
    }
    
    let mut sum_1 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for k in 0..grid[i][j].len() {
                if grid[i][j][k] == 1 {
                    for (di, dj, dk) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                        for s in [1, -1 as i32] {
                            let (ni, nj, nk) = (i as i32 + s * di, j as i32 + s * dj, k as i32 + s * dk);
                            if grid[ni as usize][nj as usize][nk as usize] == 0 {
                                sum_1 += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", sum_1);

    // fill all areas reachable by air with 2
    grid[0][1][1] = 2;
    for _ in 0..42 {
        for i in 1..(grid.len()-1) {
            for j in 1..(grid[i].len()-1) {
                for k in 1..(grid[i][j].len()-1) {
                    if grid[i][j][k] == 0 {
                        for (di, dj, dk) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                            for s in [1, -1 as i32] {
                                let (ni, nj, nk) = (i as i32 + s * di, j as i32 + s * dj, k as i32 + s * dk);
                                if grid[ni as usize][nj as usize][nk as usize] == 2 {
                                    grid[i][j][k] = 2;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut sum_2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for k in 0..grid[i][j].len() {
                if grid[i][j][k] == 1 {
                    for (di, dj, dk) in [(1, 0, 0), (0, 1, 0), (0, 0, 1)] {
                        for s in [1, -1 as i32] {
                            let (ni, nj, nk) = (i as i32 + s * di, j as i32 + s * dj, k as i32 + s * dk);
                            if grid[ni as usize][nj as usize][nk as usize] == 2 {
                                sum_2 += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Part 2: {}", sum_2);

}


fn main() {
    day18("input/input.txt")
}
