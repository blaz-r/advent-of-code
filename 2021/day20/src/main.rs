use std::fs;


fn enhance(image: &Vec<Vec<char>>, algo: &Vec<char>) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = vec![vec!['.'; image[0].len()]; image.len()];
    for i in 1..image.len()-1 {
        for j in 1..image[i].len()-1 {
            let dirs: [(i32, i32); 9] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 0), (0, 1), (1, -1), (1, 0), (1, 1)];
            let mut bits = "".to_owned();
            for (di, dj) in dirs {
                match image[(i as i32  + di) as usize][(j as i32 + dj) as usize] {
                    '#' => bits.push('1'),
                    _ => bits.push('0')
                }
            }
            let alg_i = usize::from_str_radix(&bits, 2).unwrap();
            output[i][j] = algo[alg_i];
        }
    }

    output
}


fn count(image: &Vec<Vec<char>>, offset: usize) -> usize {
    let mut sum = 0;
    for i in offset..image.len()-offset {
        for j in offset..image[i].len()-offset {
            match image[i][j] {
                '#' => sum += 1,
                _ => ()
            }
        }
    }
    sum
}


fn day20(filename: &str) {
    let data = fs::read_to_string(filename).expect("can't read file");
    let split: Vec<&str> = data.split("\n\n").collect();   
    let algo: Vec<char> = split[0].chars().collect();

    let mut image: Vec<Vec<char>> = split[1].lines()
                                           .map(|line| line.chars()
                                                           .collect()
                                                ).collect();

    let offset = 101;

    for _ in 0..offset {
        image.iter_mut().for_each(|v| { v.insert(0, '.'); v.insert(v.len(), '.')});
        image.insert(0, vec!['.'; image[0].len()]);
        image.insert(image.len(), vec!['.'; image[0].len()]);
    }

    for i in 0..50 {
        image = enhance(&image, &algo);

        if i == 1 {
            println!("PART1: {}", count(&image, offset-2));
        }
    }
    println!("PART2: {}", count(&image, offset-50));
}


fn main() {
    day20("./input/input.txt")
}
