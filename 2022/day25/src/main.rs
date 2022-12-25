use std::fs;


fn parse(num_str: &str) -> i64 {
    let mut num = 0;
    let mut mult = 1;
    for c in num_str.chars().rev() {
        let val = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("Shouldn't reach")
        };
        num += val * mult;
        mult *= 5;
    }
    num
}


fn to_snafu(mut num: i64) -> String {
    let mut str = String::new();

    while num > 0 {
        num += 2;
        let remainder = num % 5;
        match remainder {
            0 => str.insert(0, '='),
            1 => str.insert(0, '-'),
            2 => str.insert(0, '0'),
            3 => str.insert(0, '1'),
            4 => str.insert(0, '2'),
            _ => ()
        }
        num /= 5;
    }
    str
}


fn day25(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let nums: Vec<i64> = data.lines().map(|l| parse(l)).collect();
    let sum: i64 = nums.iter().sum();
    println!("Part 1: {}", to_snafu(sum));
}


fn main() {
    day25("input/input.txt");
}
