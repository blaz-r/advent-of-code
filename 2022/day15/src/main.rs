use std::fs;
use std::collections::HashSet;


fn day15(file_name: &str, row: i32, lim: i32) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let mut x_coords: HashSet<i32> = HashSet::new();
    let mut sensors: Vec<(i32, i32, i32)> = Vec::new();

    for line in data.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let sx: i32 = split[2].split("=").nth(1).unwrap().split(",").next().unwrap().parse().unwrap();
        let sy: i32 = split[3].split("=").nth(1).unwrap().split(":").next().unwrap().parse().unwrap();
        let bx: i32 = split[8].split("=").nth(1).unwrap().split(",").next().unwrap().parse().unwrap();
        let by: i32 = split[9].split("=").nth(1).unwrap().parse().unwrap();

        let manhattan = i32::abs(sx - bx) + i32::abs(sy - by);
        sensors.push((sx, sy, manhattan));
        if i32::abs(sy - row) < manhattan {
            let dx = manhattan - i32::abs(sy - row);
            {
                for x in (sx-dx)..(sx+dx) {
                    x_coords.insert(x);
                }
            }
        }
    }
    
    println!("Part 1: {}", x_coords.len());

    for (sx, sy, s_man) in sensors.iter() {
        let (mut x, mut y) = (sx - s_man - 1, *sy);
        for (dx, dy) in [(-1, 1), (1, 1), (1, -1), (-1, -1)] {
            for _r in 0..*s_man {
                if x >= 0 && x <= lim  && y >= 0 && y <= lim {
                    let mut out_of_all = true;
                    for (cx, cy, c_man) in sensors.iter() {
                        let point_man = i32::abs(x - cx) + i32::abs(y - cy);
                        out_of_all &= point_man > *c_man
                    }
                    if out_of_all {
                        println!("Part 2: {}", x as u64 * 4000000 + y as u64);
                        return;
                    }
                }
                (x, y) = (x + dx, y + dy);
            }
        }
    }
}


fn main() {
    day15("input/input.txt", 2000000, 4000000);
}
