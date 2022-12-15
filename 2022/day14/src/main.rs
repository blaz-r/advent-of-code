use std::fs;


fn day14(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");  
    let lines_coords: Vec<Vec<(usize, usize)>> = data.lines()
                                                     .map(|l| l.split(" -> ")
                                                                     .map(|coord| {
                                                                        let c_vec: Vec<&str> = coord.split(",").collect();
                                                                        (c_vec[0].parse().unwrap(), c_vec[1].parse().unwrap())
                                                                     }).collect()
                                                         ).collect(); 
    let max_y: usize = lines_coords.iter()
                                   .map(|l: &Vec<(usize, usize)>| l.iter()
                                                                   .map(|c| c.1)
                                                                   .max()
                                                                   .unwrap()
                                        ).max().unwrap();

    let mut start_grid = vec![vec!['.'; 1000]; max_y + 5];

    for line in lines_coords {
        for ((x1, y1), (x2, y2)) in line.iter().zip(line[1..].iter()) {
            if *x1 == *x2 {
                let (s, e) = if *y1 < *y2 {(*y1, *y2)} else {(*y2, *y1)};
                for y in s..=e {
                    start_grid[y][*x1] = '#';
                }
            }
            else {
                let (s, e) = if *x1 < *x2 {(*x1, *x2)} else {(*x2, *x1)};
                for x in s..=e {
                    start_grid[*y1][x] = '#';
                }
            }
        }
    } 

    let mut grid = start_grid.clone();

    let mut sand = 0;
    let mut in_abyss = false;
    while !in_abyss {
        let (mut x, mut y) = (500, 0);
        loop {
            if y >= max_y {
                in_abyss = true;
                break;
            }
            match grid[y+1][x] {
                '#' | 'o' => {
                    match grid[y+1][x-1] {
                        '#' | 'o' => {
                            match grid[y+1][x+1] {
                                '#' | 'o' => {
                                    break;
                                }
                                _ => (x, y) = (x + 1, y +1)
                            }
                        }
                        _ => (x, y) = (x - 1, y + 1)
                    }
                }
                _ => (x, y) = (x, y + 1)
            }
        }
        if !in_abyss {
            grid[y][x] = 'o';
            sand += 1;
        }
    }
    println!("Part 1: {}", sand);

    grid = start_grid.clone();
    for x in 0..grid[0].len() {
        grid[max_y + 2][x] = '#';
    }

    sand = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        loop {
            match grid[y+1][x] {
                '#' | 'o' => {
                    match grid[y+1][x-1] {
                        '#' | 'o' => {
                            match grid[y+1][x+1] {
                                '#' | 'o' => {
                                    break;
                                }
                                _ => (x, y) = (x + 1, y +1)
                            }
                        }
                        _ => (x, y) = (x - 1, y + 1)
                    }
                }
                _ => (x, y) = (x, y + 1)
            }
        }
        grid[y][x] = 'o';
        sand += 1;
        if (x, y) == (500, 0) {
            break;
        }
    }
    println!("Part 2: {}", sand);
}

fn main() {
    day14("input/input.txt");
}
