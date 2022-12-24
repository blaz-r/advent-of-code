use std::{fs, collections::{HashSet, HashMap}};


fn print_grid(grid: &HashSet<(i32, i32)>) {
    let is: Vec<i32> = grid.iter().map(|(i, _)| *i).collect();
    let js: Vec<i32> = grid.iter().map(|(_, j)| *j).collect();

    let (&min_i, &max_i) = (is.iter().min().unwrap(), is.iter().max().unwrap());
    let (&min_j, &max_j) = (js.iter().min().unwrap(), js.iter().max().unwrap());

    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if grid.contains(&(i, j)) {
                print!("#");
            }
            else {
                print!(".")
            }
        }
        println!();
    }
}


fn check_empty(grid: &HashSet<(i32, i32)>) -> i32 {
    let is: Vec<i32> = grid.iter().map(|(i, _)| *i).collect();
    let js: Vec<i32> = grid.iter().map(|(_, j)| *j).collect();

    let (&min_i, &max_i) = (is.iter().min().unwrap(), is.iter().max().unwrap());
    let (&min_j, &max_j) = (js.iter().min().unwrap(), js.iter().max().unwrap());

    return (max_i - min_i + 1) * (max_j - min_j + 1) - grid.len() as i32;
}


fn simulate(mut grid: HashSet<(i32, i32)>) {
    let neigh = [(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];
    let mut check = vec![
        ([-1, 0], [[-1, -1], [-1, 0], [-1, 1]]), // NW, N, NE
        ([1, 0], [[1, -1], [1, 0], [1, 1]]),     // SW, S, SE
        ([0, -1], [[1, -1], [0, -1], [-1, -1]]), // SW, W, NW
        ([0, 1], [[1, 1], [0, 1], [-1, 1]]),     // SE, E, NE
    ];

    let mut round = 0;
    loop {
        let mut change = false;
        let mut proposed: HashMap<(i32, i32), i32> = HashMap::new();

        for (i, j) in grid.iter() {
            if neigh.iter().any(|(di, dj)| grid.contains(&(i + di, j + dj))) {
                for ([di, dj], ch_dir) in check.iter() {
                    if ch_dir.iter().all(|[ci, cj]| !grid.contains(&(i + ci, j + cj))) {
                        let new_pos = (i + di, j + dj);
                        *proposed.entry(new_pos).or_default() += 1;
                        break;
                    }
                }
            }
        }

        let mut new: Vec<(i32, i32)> = Vec::new();
        let mut to_remove: Vec<(i32, i32)> = Vec::new();
        for (i, j) in grid.iter() {
            if neigh.iter().any(|(di, dj)| grid.contains(&(i + di, j + dj))) {
                for ([di, dj], ch_dir) in check.iter() {
                    if ch_dir.iter().all(|[ci, cj]| !grid.contains(&(i + ci, j + cj))) {
                        let new_pos = (i + di, j + dj);
                        if proposed.contains_key(&new_pos) && proposed[&new_pos] == 1 {
                            change = true;
                            new.push(new_pos);
                            to_remove.push((*i, *j));
                        }
                        break;
                    }
                }
            }
        }

        for r in to_remove {
            grid.remove(&r);
        }
        for n in new {
            grid.insert(n);
        }

        round += 1;
        if round == 10 {
            println!("Part 1: {}", check_empty(&grid));
        }

        check.rotate_left(1);

        if !change {
            break;
        }
    }
    println!("Part 2: {}", round);

}


fn day23(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    data.lines()
        .enumerate()
        .for_each(|(i, l)| l.chars()
                                         .enumerate()
                                         .filter(|(_, c)| *c == '#')
                                         .for_each(|(j, _)| {grid.insert((i as i32, j as i32));})
                 );  

    simulate(grid);
}


fn main() {
    day23("input/input.txt");
}
