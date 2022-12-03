use std::fs;


fn dirac_dice(p1: usize, p2: usize, p1_score: usize, p2_score: usize, turn: usize, memo: &mut Vec<Vec<Vec<Vec<Vec<(usize, usize)>>>>>) -> (usize, usize) {
    if p1_score >= 21 {
        return (1, 0);
    }
    else if p2_score >= 21 {
        return (0, 1);
    }

    if memo[turn][p1_score][p2_score][p1][p2] != (0, 0) {
        return memo[turn][p1_score][p2_score][p1][p2];
    }

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for die1 in 1..4 {
        for die2 in 1..4 {
            for die3 in 1..4 {
                match turn % 2 {
                    0 => {
                        let die_roll = die1 + die2 + die3;
                        let p1_1 = (p1 + die_roll - 1) % 10 + 1;
            
                        let (p1_w_ret, p2_w_ret) = dirac_dice(p1_1, p2, p1_score + p1_1, p2_score, turn + 1, memo);
                        p1_wins += p1_w_ret;
                        p2_wins += p2_w_ret;

                    }
                    _ => {
                        let die_roll = die1 + die2 + die3;
                        let p2_1 = (p2 + die_roll - 1) % 10 + 1;
            
                        let (p1_w_ret, p2_w_ret) = dirac_dice(p1, p2_1, p1_score, p2_score + p2_1, turn + 1, memo);
                        p1_wins += p1_w_ret;
                        p2_wins += p2_w_ret;
                    }
                }
            }
        }
    }

    memo[turn][p1_score][p2_score][p1][p2] = (p1_wins, p2_wins);

    (p1_wins, p2_wins)
}


fn day21(filename: &str) {
    let data = fs::read_to_string(filename).expect("lalalal :)");
    let lines: Vec<&str> = data.lines().map(|line| line.split(":").nth(1).unwrap()).collect();
    let mut player1 = lines[0].trim().parse::<usize>().unwrap();
    let mut player2 = lines[1].trim().parse::<usize>().unwrap();

    let mut memo = vec![vec![vec![vec![vec![(0, 0); 11]; 11]; 22]; 22]; 42];
    let p2 = dirac_dice(player1, player2, 0, 0, 0, &mut memo);

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut die = 1;

    let mut turn = 0;
    loop {
        let mut die_roll = 0;
        for _ in 0..3 {
            die_roll += die;
            die = die % 100 + 1;
        }
        match turn % 2 {
            0 => {
                player1 = (player1 + die_roll - 1) % 10 + 1;
                p1_score += player1;

                if p1_score >= 1000 {
                    println!("PART1: {}", p2_score * (turn + 1) * 3);
                    break;
                }
            }
            _ => {
                player2 = (player2 + die_roll - 1) % 10 + 1;
                p2_score += player2;

                if p2_score >= 1000 {
                    println!("PART1: {}", p1_score * (turn + 1) * 3);
                    break;
                }
            }
        }
        turn += 1;
    }

    println!("PART2: {}", std::cmp::max(p2.0, p2.1));
}


fn main() {
    day21("./input/input.txt")
}
