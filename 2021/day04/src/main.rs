use std::fs;


struct BingoBoard {
    board_num: [[i32; 5]; 5],
    board_picked: [[bool; 5]; 5]
}


fn build_bingo_boards(lines_in: Vec<&str>, size: usize) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = Vec::with_capacity(size);

    for i in (1..lines_in.len()).step_by(5) {
        let mut new_board = BingoBoard { board_num: [[0; 5]; 5], board_picked: [[false; 5]; 5] };
        for j in 0..5 {
            let curr_numbers: Vec<i32> = lines_in[i + j].split_whitespace().map(|s_num| s_num.parse().unwrap()).collect();
            new_board.board_num[j] = curr_numbers.try_into().unwrap();
        }
        boards.push(new_board);
    }
    boards
}


fn mark_if_present(board: &mut BingoBoard, selected_num: i32) {
    for (i, row) in board.board_num.into_iter().enumerate() {
        for (j, board_num) in row.into_iter().enumerate() {
            if board_num == selected_num {
                board.board_picked[i][j] = true;
            }
        }
    }
}


fn is_winning(board: &BingoBoard) -> bool {
    // check row
    for row in board.board_picked.into_iter() {
        let all =  row.iter().fold(0, |sum, &val| sum + val as i32);
        if all == 5 {
            return true
        }
    }
    // check column
    for i in 0..5 {
        let mut all = 0;
        for j in 0..5 {
            all += board.board_picked[j][i] as i32;
        }
        if all == 5 {
            return true
        }
    }
    false
}


fn sum_of_unmarked(board: &BingoBoard) -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
           if !board.board_picked[i][j] {
                sum += board.board_num[i][j];
           }
        }
    }
    sum
}


fn day1(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let lines = data.lines().filter(|&line| line != "").collect::<Vec<_>>();

    let draw: Vec<i32> = lines[0].split(",").map(|s_num| s_num.parse().unwrap()).collect();
    let mut boards = build_bingo_boards(lines, draw.len());

    let mut boards_won =  vec![false; boards.len()];

    let mut found_first = false;
    let mut found_last: i32 = -1;
    for num in draw {
        for (i, board) in boards.iter_mut().enumerate() {
            mark_if_present(board, num);

            if is_winning(board){
                if !found_first {
                    println!("PART1: {}", sum_of_unmarked(board) * num);
                    found_first = true;
                }

                let last = boards_won.iter().position(|&v| !v).unwrap();
                boards_won[i] = true;
                let all = boards_won.iter().fold(0, |sum, &val| sum + val as i32);

                if all == boards_won.len() as i32 {
                    found_last = last as i32;
                    break;
                }
            }
        }

        if found_last > 0 {
            println!("PART2: {}", sum_of_unmarked(&boards[found_last as usize]) * num);
            break;
        }
    }
}


fn main() {
    day1("./input/input.txt")
}
