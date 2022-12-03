use std::fs;
use std::collections::HashMap;


fn day02(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file.");
    let outcome = HashMap::from([
        ("lose", 0),
        ("draw", 3),
        ("win", 6),

        // part 2
        // lose
        ("X", 0),
        // draw
        ("Y", 3),
        // win
        ("Z", 6)
    ]);
    let move_score = HashMap::from([
        // rock
        ("X", 1),
        // paper
        ("Y", 2),
        // scissors
        ("Z", 3)
    ]);

    let mut sum_1 = 0;
    let mut sum_2 = 0;
    for turn in data.lines() {
        let split: Vec<&str> = turn.split(" ").collect();
        
        let (res_1, move_2) = match split[0] {
            // rock
            "A" => {
                match split[1] {
                    "Y" => ("win", "X"),
                    "Z" => ("lose", "Y"),
                    _ => ("draw", "Z")
                }
            }
            //paper
            "B" => {
                match split[1] {
                    "Z" => ("win", "Z"),
                    "X" => ("lose", "X"),
                    _ => ("draw", "Y")
                }
            }
            // scissors C
            _ => {
                match split[1] {
                    "X" => ("win", "Y"),
                    "Y" => ("lose", "Z"),
                    _ => ("draw", "X")
                }
            }
        };
        sum_1 += move_score.get(split[1]).unwrap();
        sum_1 += outcome.get(res_1).unwrap();

        sum_2 += move_score.get(move_2).unwrap();
        sum_2 += outcome.get(split[1]).unwrap();
    }
    
    println!("Part 1: {}", sum_1);
    println!("Part 2: {}", sum_2);
}


fn main() {
    day02("./input/input.txt");
}
