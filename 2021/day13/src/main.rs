use std::fs;


fn fold_y(paper: &Vec<Vec<bool>>, index: usize) -> Vec<Vec<bool>> {
    let mut lower = vec![vec![]; paper.len() / 2];
    lower.clone_from_slice(&paper[index+1..]);
    lower.reverse();

    for i in 0..lower.len() {
        for j in 0..lower[i].len() {
            lower[i][j] |= paper[i][j];
        }
    }
    lower
}


fn fold_x(paper: &Vec<Vec<bool>>, index: usize) -> Vec<Vec<bool>> {
    let mut left = vec![vec![false; paper[0].len() / 2]; paper.len()];
    for i in 0..paper.len() {
        left[i].clone_from_slice(&paper[i][index+1..]);
        left[i].reverse();
    }

    for i in 0..left.len() {
        for j in 0..left[i].len() {
            left[i][j] |= paper[i][j];
        }
    }

    left
}


fn day13(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();

    let mut pairs: Vec<Vec<usize>> = vec![];
    let mut fold: Vec<(&str, usize)> = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in data.lines() {
        if line.starts_with("fold") {
            let instr: Vec<&str> = line.split(" ").nth(2).unwrap().split("=").collect();
            fold.push((instr[0], instr[1].parse().unwrap()));
        }
        else {
            if !line.is_empty() {
                let pair: Vec<usize> = line.split(",").map(|num| num.parse::<usize>().unwrap()).collect();
                if pair[0] > width { width = pair[0]};
                if pair[1] > height { height = pair[1]};
                pairs.push(pair);
            }
        }
    }
    
    let mut paper: Vec<Vec<bool>> = vec![vec![false; width + 1]; height + 1];
    pairs.iter().for_each(|p| paper[p[1]][p[0]] = true);

    let mut first = true;
    for fold in fold {
        if fold.0 == "x" {
            paper = fold_x(&paper, fold.1);
        }
        else {
            paper = fold_y(&paper, fold.1);
        }
        if first {
            println!("PART1: {}", paper.iter().flatten().filter(|&&dot| dot).count());
            first = false;
        }
    }
    for row in paper {
        for el in row {
            if el { print!("#") }
            else { print!("_") }
        }
        println!();
    }
}


fn main() {
    day13("./input/input.txt")
}
