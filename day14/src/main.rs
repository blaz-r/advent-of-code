use std::fs;
use std::collections::HashMap;


fn day14(filename: &str, cycles: i32) -> i64 {
    let data = fs::read_to_string(filename).unwrap();
    let mut split = data.split("\n\n");
    let poly_str: Vec<char> = split.next().unwrap().chars().collect();
    // poly is made of adjecent chars (LMOA -> LM, MO, OA)
    let mut poly: HashMap<String, i64> = HashMap::new();
    for i in 0..poly_str.len()-1 {
        let current: String = poly_str[i..i+2].iter().collect();
        *poly.entry(current).or_insert(0) += 1;
    }
    // paterns are of form {patern: insert_char}
    let mut patterns: HashMap<String, char> = HashMap::new();
    for line in split.next().unwrap().lines() {
        let pair: Vec<&str> = line.split(" -> ").collect();
        patterns.insert(pair[0].to_owned(), pair[1].chars().nth(0).unwrap());
    }

    for _ in 0..cycles {
        let mut temp = poly.clone();
        for (pat, c) in patterns.iter() {
            if poly.contains_key(pat) {
                let pat_chars: Vec<char> = pat.chars().collect();
                // if we insert X into YZ we get YX and XZ
                let first = format!("{}{}", c, pat_chars[1]);
                let second = format!("{}{}", pat_chars[0], c);

                *temp.entry(first).or_insert(0) += poly[pat];
                *temp.entry(second).or_insert(0) += poly[pat];
            }
        }
        // remove all occurances of same pattern in previous poly
        for k in poly.keys() {
            if temp.contains_key(k) {
                *temp.get_mut(k).unwrap() -= poly.get(k).unwrap(); 
            }
        }
        poly = temp;
    }
    // every char except 1st and last appears twice
    let mut occurance: HashMap<char, i64> = HashMap::new();
    for (pt, count) in poly.iter() {
        let pt_chars: Vec<char> = pt.chars().collect();
        *occurance.entry(pt_chars[0]).or_insert(0) += count;
        *occurance.entry(pt_chars[1]).or_insert(0) += count;
    }
    // fix 1st and last to also appear twice
    *occurance.get_mut(&poly_str[0]).unwrap() += 1;
    *occurance.get_mut(poly_str.last().unwrap()).unwrap() += 1;

    occurance.values().max().unwrap() / 2 - occurance.values().min().unwrap() / 2
}


fn main() {
    println!("PART1: {}", day14("./input/input.txt", 10));
    println!("PART2: {}", day14("./input/input.txt", 40));
}
