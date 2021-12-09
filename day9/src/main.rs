use std::fs;


fn basin_size(heightmap: &mut Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    if i < 1 || j < 1 || heightmap[i as usize][j as usize] >= 9 {
        return 0;
    }

    heightmap[i][j] = 10;
    let mut size = 1;
    size += basin_size(heightmap, i - 1, j);
    size += basin_size(heightmap, i + 1, j);
    size += basin_size(heightmap, i, j - 1);
    size += basin_size(heightmap, i, j + 1);

    size
}


fn day9(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let mut heightmap: Vec<Vec<i32>> = data.lines()
                                           .map(|line| line.chars()
                                                           .map(|c| c.to_digit(10).unwrap() as i32)
                                                           .collect::<Vec<i32>>()
                                                ).collect();

    // pad all borders with 10
    heightmap.iter_mut().for_each(|v| { v.insert(0, 10); v.insert(v.len(), 10)});
    heightmap.insert(0, vec![10; heightmap[0].len()]);
    heightmap.insert(heightmap.len(), vec![10; heightmap[0].len()]);

    let mut sum = 0;
    let mut minimums : Vec<(usize, usize)> = vec![];
    for i in 1..heightmap.len()-1 {
        for j in 1..heightmap[i].len()-1 {

            let neigh_4 = [heightmap[i][j], heightmap[i - 1][j], heightmap[i + 1][j], heightmap[i][j + 1], heightmap[i][j - 1]];
            let unique = neigh_4.iter().filter(|&&num| num == heightmap[i][j]).count() == 1;

            if &heightmap[i][j] == neigh_4.iter().min().unwrap() && unique {
                sum += heightmap[i][j] + 1;  
                minimums.push((i, j));        
            }
        }
    }

    println!("PART1: {}", sum);

    let mut basins: Vec<i32> = vec![];
    for (i, j) in minimums.iter() {
        basins.push(basin_size(&mut heightmap, *i, *j));
    }

    basins.sort_by(|a, b| b.cmp(a));
    let max = basins[0..3].iter().fold(1, |prod, num| prod * num);

    println!("PART2: {}", max);
}


fn main() {
    day9("./input/input.txt");
}
