use std::fs;


fn advance(cucumbers: &Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    let mut new_cucumbers = vec![vec!['.'; cucumbers[0].len()]; cucumbers.len()];
    let mut change = false;
    for (i, row) in cucumbers.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '>' {
                let next_pos = (j + 1) % cucumbers[i].len();
                if cucumbers[i][next_pos] == '.' {
                    new_cucumbers[i][next_pos] = '>';
                    change = true;
                }
                else {
                    new_cucumbers[i][j] = '>';
                }
            }
            else if c == 'v' {
                new_cucumbers[i][j] = 'v';
            }
        }
    }

    let mut final_cucumbers: Vec<Vec<char>> = new_cucumbers.iter()
                                                           .map(|row| row.clone()
                                                               ).collect();

    for (i, row) in cucumbers.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == 'v' {
                let next_pos = (i + 1) % cucumbers.len();
                if new_cucumbers[next_pos][j] == '.'{
                    final_cucumbers[next_pos][j] = 'v';
                    final_cucumbers[i][j] = '.';
                    change = true;
                }
            }
        }
    }

    match change {
        true => Some(final_cucumbers),
        false => None
    }
}


fn day25(filename: &str) {
    let data = fs::read_to_string(filename).expect("Merry christmas :)");

    let mut cucumbers: Vec<Vec<char>> = data.lines()
                                            .map(|line| line.chars()
                                                            .collect()
                                                ).collect();
    let mut i = 1;
    while let Some(next_cucumbers) = advance(&cucumbers) {
        cucumbers = next_cucumbers;
        i += 1;
    }
    println!("PART1: {}", i);
}


fn main() {
    day25("./input/input.txt");
}
