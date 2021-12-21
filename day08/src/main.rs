use std::fs;


fn day8(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let lines = data.lines().map(|line| line.split(" | ").collect()).collect::<Vec<Vec<&str>>>();

    let mut appearance = 0;
    for line in lines.iter() {
        let output = line[1].split(" ").collect::<Vec<&str>>();
        appearance += output.iter().map(|out_seg| { match out_seg.len() {
                                                        2 | 3 | 4 | 7 => 1,
                                                        _ => 0}}
                                                    ).sum::<i32>();
    }
    println!("PART1: {}", appearance);

    let mut translation = vec![""; 10];
    let mut sum = 0;

    for line in lines.iter() {
        let output = line[1].split(" ").collect::<Vec<&str>>();
        let numbers = line[0].split(" ").collect::<Vec<&str>>();

        for num in numbers.iter() {
            match num.len() {
                2 => translation[1] = num,
                3 => translation[7] = num,
                4 => translation[4] = num,
                7 => translation[8] = num,
                _ => ()
            }
        }

        for num in numbers.iter() {
            let s_len = num.len();
            match s_len {
                2 | 3 | 4 | 7 => (),
                6 => {
                    if !translation[1].chars().all(|c| num.contains(c)) {
                        // 6 -> 8-1 ne 1
                        translation[6] = num;
                    }
                    else {
                        if translation[4].chars().all(|c| num.contains(c)) {
                            // 9 -> 8-1 in 1 in 4
                            translation[9] = num;
                        }
                        else {
                            // 0 -> 8-1 ne 4 in 1
                            translation[0] = num;
                        }
                    }
                },
                5 => {
                    if translation[1].chars().all(|c| num.contains(c)) {
                        // 3 -> 8-2 in 1
                        translation[3] = num;
                    }
                    else {
                        if translation[4].chars().map(|c| if num.contains(c) {1} else {0}).sum::<i32>() == 3 {
                            // 5 -> 8-2 3 od 4
                            translation[5] = num;
                        }
                        else {
                            // 2 -> 8-2 2 od 4
                            translation[2] = num;
                        }
                    }
                },
                _ => ()
            }
        }

        let mut result = 0;
        for num in output.iter() {
            for (i, tr) in translation.iter().enumerate() {
                if tr.len() == num.len() && tr.chars().all(|c| num.contains(c)) {
                    result *= 10;
                    result += i;
                }
            }
        }
        sum += result;
    }

    println!("PART2: {}", sum);
}


fn main() {
    day8("./input/input.txt");
}
