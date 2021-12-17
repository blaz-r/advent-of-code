use std::fs;


fn day17(filename: &str) {
    let data = fs::read_to_string(filename).expect("can't read file");
    let area: Vec<&str> = data.split(":").nth(1).unwrap().split(",").collect();
    let x_range: Vec<i32> = area[0].split("=").nth(1).unwrap().split("..").map(|num| num.parse::<i32>().unwrap()).collect();
    let y_range: Vec<i32> = area[1].split("=").nth(1).unwrap().split("..").map(|num| num.parse::<i32>().unwrap()).collect();

    let mut max = 0;
    let mut count = 0;
    for init_vx in 0..420 {
        for init_vy in -420..420 {
            let mut vx = init_vx;
            let mut vy = init_vy;
            let mut pos_x = 0;
            let mut pos_y = 0;

            let mut curr_max = 0;
            loop {
                pos_x += vx;
                pos_y += vy;

                if pos_y > curr_max {
                    curr_max = pos_y;
                }

                if pos_x >= x_range[0] && pos_x <= x_range[1] && pos_y >= y_range[0] && pos_y <= y_range[1] {
                    if curr_max > max {
                        max = curr_max;
                    }
                    count += 1;
                    break;
                }
                else if pos_x > x_range[1] || pos_y < y_range[0] {
                    break;
                }
                else {
                    if vx > 0 {
                        vx -= 1;
                    }
                    vy -= 1;
                }
            }
        }
    }
    println!("PART1: {}", max);
    println!("PART2: {}", count);
}


fn main() {
    day17("./input/input.txt");
}
