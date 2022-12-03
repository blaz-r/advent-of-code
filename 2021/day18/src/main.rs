use std::fs;
use std::fmt;

#[derive(Clone)]
struct SnailNum {
    val: u32,
    nest: u32
}


impl fmt::Debug for SnailNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.val, self.nest)
    }
}


fn parse_num(line: &Vec<char>) -> Vec<SnailNum> {
    let mut num = Vec::new();
    let mut nest = 0;
    for c in line {
        match c {
            '[' => nest += 1,
            ']' => nest -= 1,
            n => if n.is_digit(10) { num.push( SnailNum{val: n.to_digit(10).unwrap(), nest: nest}) }
        }
    }

    num
}


fn explode(num: &mut Vec<SnailNum>) -> bool {
    for i in 1..num.len() {
        if num[i-1].nest == 5 && num[i].nest == 5 {
            if i > 1 {
                num[i - 2].val += num[i - 1].val;
            }
            if i + 1 < num.len() {
                num[i + 1].val += num[i].val;
            }
            num[i - 1].val = 0;
            num[i - 1].nest -= 1;

            num.remove(i);

            return true;
        }
    }
    false
}


fn split(num: &mut Vec<SnailNum>) -> bool {
    for i in 0..num.len() {
        if num[i].val >= 10 {
            let temp = num[i].val;
            num[i].val /= 2;
            num[i].nest += 1;

            num.insert(i+1, SnailNum{val: temp - num[i].val, nest: num[i].nest});

            return true;
        }
    }
    false
}


fn add(sum: &mut Vec<SnailNum>, num2: &Vec<SnailNum>) {
    sum.iter_mut().for_each(|n| n.nest += 1);
    (0..num2.len()).for_each(|i| sum.push(SnailNum{val: num2[i].val, nest: num2[i].nest + 1}));

    while explode(sum) || split(sum) { }
}


fn magnitude(num: &mut Vec<SnailNum>) -> u32 {
    let mut summed = true;
    while summed && num.len() > 1 {
        summed = false;
        let mut i = 0;
        while i < num.len()-1 {
            if num[i].nest == num[i + 1].nest {
                num[i].val = 3 * num[i].val + 2 * num[i + 1].val;
                num[i].nest -= 1;

                summed = true;
                num.remove(i + 1);
            }
            i += 1;
        }
    }

    num[0].val
}


fn max_mag(numbers: &mut Vec<Vec<SnailNum>>) -> u32 {
    let mut max = 0;

    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                // ugly stuff, but at this point I'm sick of this thing :D
                let mut sum1 = numbers[i].clone();
                let mut sum2 = numbers[j].clone();
                add(&mut sum1, &numbers[j]);
                add(&mut sum2, &numbers[i]);
                let mag1 = magnitude(&mut sum1);
                let mag2 = magnitude(&mut sum2);
                
                if mag1 > max { max = mag1};
                if mag2 > max { max = mag1};
            }
        }
    }

    max
}


fn day18(filename: &str) {
    let data = fs::read_to_string(filename).expect("can't read file");
    let lines: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

    let mut numbers: Vec<Vec<SnailNum>> = lines.iter().map(|line| parse_num(line)).collect();

    let mut sum = numbers[0].clone();

    for i in 1..numbers.len() {
        add(&mut sum, &mut numbers[i]);
    }

    println!("PART1: {}", magnitude(&mut sum));

    println!("PART2: {}", max_mag(&mut numbers));
}


fn main() {
    day18("./input/input.txt")
}
