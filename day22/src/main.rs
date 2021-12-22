use std::fs;
use std::collections::HashSet;


fn solve(instructions: &Vec<(&str, [i64; 6])>) -> usize {
    let mut x_set: HashSet<i64> = HashSet::new();
    let mut y_set: HashSet<i64> = HashSet::new();
    let mut z_set: HashSet<i64> = HashSet::new();

    for (_, coords) in instructions.iter() {
        x_set.insert(coords[0]);
        x_set.insert(coords[1] + 1);
        y_set.insert(coords[2]);
        y_set.insert(coords[3] + 1);
        z_set.insert(coords[4]);
        z_set.insert(coords[5] + 1);
    }

    let mut grid = vec![vec![vec![false; z_set.len() - 1]; y_set.len() - 1]; x_set.len() - 1];

    let mut x_vec = Vec::from_iter(x_set);
    let mut y_vec = Vec::from_iter(y_set);
    let mut z_vec = Vec::from_iter(z_set);
    x_vec.sort();
    y_vec.sort();
    z_vec.sort();


    for (state, coords) in instructions {
        let x1 = x_vec.iter().position(|&el| el == coords[0]).unwrap();
        let x2 = x_vec.iter().position(|&el| el == coords[1] + 1).unwrap();
        let y1 = y_vec.iter().position(|&el| el == coords[2]).unwrap();
        let y2 = y_vec.iter().position(|&el| el == coords[3] + 1).unwrap();
        let z1 = z_vec.iter().position(|&el| el == coords[4]).unwrap();
        let z2 = z_vec.iter().position(|&el| el == coords[5] + 1).unwrap();

        for x in x1..x2 {
            for y in y1..y2 {
                for z in z1..z2 {
                    grid[x][y][z] = *state == "on";
                }
            }
        }
    }

    let mut count: usize = 0;
    for ix in 0..grid.len() {
        for iy in 0..grid[ix].len() {
            for iz in 0..grid[ix][iy].len() {
                if grid[ix][iy][iz] {
                    count += ((x_vec[ix + 1] - x_vec[ix]) * (y_vec[iy + 1] - y_vec[iy]) * (z_vec[iz + 1] - z_vec[iz])) as usize;
                }
            }
        }
    }

    count
}


fn day22(filename: &str) {
    let data = fs::read_to_string(filename).expect("can't read file");
    let state_coords_split: Vec<Vec<&str>> = data.lines().map(|line| line.split(" ").collect()).collect();
    let mut instructions: Vec<(&str, [i64; 6])> = Vec::new();
    let mut part1_instr: Vec<(&str, [i64; 6])> = Vec::new();

    for l in state_coords_split.iter() {
        let state = l[0];
        let coords: Vec<Vec<i64>> = l[1].split(",").map(|c| c[2..].split("..")
                                                   .map(|n| n.parse::<i64>().unwrap()).collect()
                                        ).collect();
        let instr = (state, [coords[0][0], coords[0][1], coords[1][0], coords[1][1], coords[2][0], coords[2][1]]);
        instructions.push(instr);
        
        if coords[0][0] > -50 && coords[0][0] < 50 {
            part1_instr.push(instr);
        }
    }

    println!("PART1: {}", solve(&part1_instr));
    println!("PART2: {}", solve(&instructions));
}


fn main() {
    day22("./input/input.txt");
}
