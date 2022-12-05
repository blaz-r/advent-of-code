use std::fs;


fn make_stack(stack_rows: &Vec<&str>, n_stacks: usize) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec![]; n_stacks];
    for r in 0..(stack_rows.len()-1) {
        for i in 0..n_stacks {
            let item = if i == n_stacks-1 {
                stack_rows[r][i*4..i*4+3].chars().nth(1).unwrap()
            } 
            else {
                stack_rows[r][i*4..(i+1)*4].chars().nth(1).unwrap()
            };
            if item != ' ' {
                stacks[i].insert(0, item);
            }
        }
    }
    stacks
}


fn day05(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let stack_split: Vec<&str> = data.split("\n\n").collect();

    let stack_rows: Vec<&str> = stack_split[0].lines().collect();
    let n_stacks = (stack_rows[0].chars().count() + 1) / 4;
    
    let mut stacks: Vec<Vec<char>> = make_stack(&stack_rows, n_stacks);
    
    for m_str in stack_split[1].lines() {
        let m_split: Vec<&str> = m_str.split(" ").collect();
        // move <times> from <src> to <dst>
        let times: usize = m_split[1].parse().unwrap();
        let src: usize = m_split[3].parse().unwrap();
        let dst: usize = m_split[5].parse().unwrap();

        for _ in 0..times {
            let m_item = stacks[src-1].pop().unwrap();
            stacks[dst-1].push(m_item);
        }
    }
    print!("Part 1: ");
    for i in 0..n_stacks {
        print!("{}", stacks[i].last().unwrap());
    }
    println!();

    stacks = make_stack(&stack_rows, n_stacks);

    for m_str in stack_split[1].lines() {
        let m_split: Vec<&str> = m_str.split(" ").collect();
        // move <times> from <src> to <dst>
        let times: usize = m_split[1].parse().unwrap();
        let src: usize = m_split[3].parse().unwrap();
        let dst: usize = m_split[5].parse().unwrap();

        let src_len = stacks[src-1].len();
        let mut m_items: Vec<char> = stacks[src-1].drain((src_len-times)..).collect();
        stacks[dst-1].append(&mut m_items);
    }
    print!("Part 2: ");
    for i in 0..n_stacks {
        print!("{}", stacks[i].last().unwrap());
    }
    println!();
}


fn main() {
    day05("input/input.txt")
}
