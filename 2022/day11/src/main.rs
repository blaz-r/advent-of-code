use std::fs;
use Oper::{Mul, Sum};
use OperVal::{Old, Num};

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}


#[derive(Debug, Clone, Copy)]
enum OperVal {
    Old,
    Num(u64)
}

#[derive(Debug, Clone, Copy)]
enum Oper {
    Mul(OperVal, OperVal),
    Sum(OperVal, OperVal)
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Oper,
    divisor: u64,
    true_pass: usize,
    false_pass: usize
}

fn day11(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let mut monkeys_start: Vec<Monkey> = Vec::new();

    for monkey_block in data.split("\n\n") {
        let m_lines: Vec<&str> = monkey_block.lines().collect();
        let items: Vec<u64> = m_lines[1].split(":")
                              .nth(1).unwrap()
                              .split(", ")
                              .map(|n| n.trim()
                                              .parse::<u64>()
                                            .unwrap()
                                  ).collect();
        let oper_parts: Vec<&str> = m_lines[2].split(" = ")
                                              .nth(1)
                                              .unwrap()
                                              .split(" ").collect();

        let right = if oper_parts[2] == "old" {
            Old
        }
        else {
            Num(oper_parts[2].parse().unwrap())
        };

        let operation = if oper_parts[1] == "*" {
            Mul(Old, right)
        }
        else {
            Sum(Old, right)
        };
        let divisor: u64 = m_lines[3].split(" ")
                                     .last()
                                     .unwrap()
                                     .parse()
                                     .unwrap();
        let true_pass: usize = m_lines[4].split(" ")
                                  .last()
                                  .unwrap()
                                  .parse()
                                  .unwrap();
        let false_pass: usize = m_lines[5].split(" ")
                                       .last()
                                       .unwrap()
                                       .parse()
                                       .unwrap();

        let monkey = Monkey{
            items: items,
            operation: operation,
            divisor: divisor,
            true_pass: true_pass,
            false_pass: false_pass
        };

        monkeys_start.push(monkey);
    }

    let mut modulo = 1;
    for m in &monkeys_start {
        modulo = lcm(modulo, m.divisor);
    }

    for part_r in [20, 10000] {
        let mut monkeys = monkeys_start.clone();

        let mut inspections: Vec<u64> = vec![0; monkeys.len()];
        for _r in 0..part_r {
            for m_i in 0..monkeys.len() {
                let m = monkeys[m_i].clone();
                for item in m.items {
                    inspections[m_i] += 1;
    
                    let mut new_worry  = match &m.operation {
                        Mul(_, val) => {
                            match val {
                                Old => item * item,
                                Num(n) => item * n
                            }
                        }
                        Sum(_, val) => {
                            match val {
                                Old => item + item,
                                Num(n) => item + n
                            }
                        }
                    };

                    if part_r == 20 {
                        new_worry /= 3;
                    }
                    else {
                        new_worry %= modulo;
                    }
    
                    if new_worry % m.divisor == 0 {
                        monkeys[m.true_pass].items.push(new_worry);
                    }
                    else {
                        monkeys[m.false_pass].items.push(new_worry);
                    }
                }
                monkeys[m_i].items.clear();
            }
        }
        inspections.sort_by(|a, b| b.cmp(a));

        if part_r == 20 {
            println!("Part 1: {}", inspections[0] * inspections[1]);
        }
        else {
            println!("Part 2: {}", inspections[0] * inspections[1]);
        }
    }
}


fn main() {
    day11("input/input.txt");
}
