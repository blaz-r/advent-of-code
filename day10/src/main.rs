use std::fs;
use std::collections::HashMap;


fn day10(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = data.lines().collect();

    let weight = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let closing = HashMap::from([(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let points = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut stack: Vec<char> = vec![];
    let mut sum = 0;
    let mut completion: Vec<i64> = vec![];

    for line in lines.iter() {
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                c =>    if stack.pop().unwrap() != closing[&c] { 
                            sum += weight[&c];
                            stack.clear();
                            break;
                        }
            }
        }

        let mut score: i64 = 0;
        while !stack.is_empty() {
            let stack_el = stack.pop().unwrap();
            score *= 5;
            score += points[&stack_el];
        }
        if score > 0 {completion.push(score)};
    }
    completion.sort();
    
    println!("PART1: {}", sum);
    println!("PART2: {}", completion[completion.len() / 2])
}


fn main() {
    day10("./input/input.txt")
}
