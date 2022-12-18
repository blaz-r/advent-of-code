use std::fs;
use std::collections::HashMap;


fn collide(cave: &Vec<Vec<char>>, shape: &[[char; 4]; 4], x: usize, y: usize) -> bool {
    let cave_y0 = cave.len() - 1;
    for (i, row) in shape.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el == '#' && cave[cave_y0 - y - 3 + i][x + j] == '#' {
                return true;
            }
        }
    }
    return false;
}


fn place(cave: &mut Vec<Vec<char>>, shape: &[[char; 4]; 4], x: usize, y: usize) {
    let cave_y0 = cave.len() - 1;
    for (i, row) in shape.iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el == '#' {
                cave[cave_y0 - y - 3 + i][x + j] = '#';
            }
        }
    }
}


fn print_cave(cave: &Vec<Vec<char>>, lim: usize) {
    for i in (cave.len() - lim)..cave.len() {
        for j in 0..cave[i].len() {
            print!("{}", cave[i][j]);
        }
        println!();
    }
    println!("-------");
}


fn day18(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let jets: Vec<char> = data.chars().collect();

    let shapes: [[[char; 4]; 4]; 5] = [
        [['.', '.', '.', '.'], ['.', '.', '.', '.'], ['.', '.', '.', '.'], ['#', '#', '#', '#']],
        [['.', '.', '.', '.'], ['.', '#', '.', '.'], ['#', '#', '#', '.'], ['.', '#', '.', '.']],
        [['.', '.', '.', '.'], ['.', '.', '#', '.'], ['.', '.', '#', '.'], ['#', '#', '#', '.']],
        [['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '.'], ['#', '.', '.', '.']],
        [['.', '.', '.', '.'], ['.', '.', '.', '.'], ['#', '#', '.', '.'], ['#', '#', '.', '.']]
    ];
    let shapes_height: [usize; 5] = [1, 3, 3, 4, 2]; 
    let shapes_width: [usize; 5] = [4, 3, 3, 1, 2]; 

    let mut cave: Vec<Vec<char>> = vec![vec!['.'; 7]; 20000];
    let mut cycle_map: HashMap<(usize, usize, String), (usize, usize)> = HashMap::new();

    let mut jet_i = 0;
    let mut shape_i = 0;
    let mut highest = 0;
    let mut sim_height = 0;

    let mut l = 0;
    let mut cycle_detected = false;
    let mut part1 = true;
    while l < 4000 {

        let (mut x, mut y) = (2, highest + 3);
        loop {
            match jets[jet_i] {
                '<' => {
                    let nx = i32::max(0, x - 1);
                    if !collide(&cave, &shapes[shape_i], nx as usize, y) {
                        x = nx;
                    }
                }
                '>' => {
                    let nx = i32::min(7 - shapes_width[shape_i] as i32, x + 1);
                    if !collide(&cave, &shapes[shape_i], nx as usize, y) {
                        x = nx;
                    }
                }
                _ => ()
            }
            jet_i = (jet_i + 1) % jets.len();

            if y == 0 || collide(&cave, &shapes[shape_i], x as usize, y - 1) {
                place(&mut cave, &shapes[shape_i], x as usize, y);
                break;
            }
            else {
                y = y - 1;
            }

        }

        highest = usize::max(y + shapes_height[shape_i], highest);
        shape_i = (shape_i + 1) % shapes.len();
        l += 1;

        if l == 2022 && part1 {
            println!("Part 1: {}", highest);
            part1 = false;
        }

        if !cycle_detected && highest > 1 && !part1 {
            let key: (usize, usize, String) = (shape_i, jet_i, cave[highest-1].iter().collect());
            match cycle_map.get(&key) {
                Some((prev_high, prev_l)) => {
                    let rocks_diff = l - prev_l;
                    let height_diff = highest - prev_high;

                    let remaining_cycles = (1000000000000 - l) / rocks_diff - 1;
                    sim_height = remaining_cycles * height_diff;
                    let remaining_rocks = 1000000000000 - l - remaining_cycles * rocks_diff;
                    l = 4000 - remaining_rocks;

                    cycle_detected = true;
                },
                None => {cycle_map.insert(key, (highest, l));}
            }
        }
    }
    println!("Part 2: {}", highest + sim_height);
}


fn main() {
    day18("input/input.txt");
}
