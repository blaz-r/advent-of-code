use std::fs;


fn day06(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let buffer: Vec<char> = data.chars().collect();

    for i in 4..buffer.len() {
        let window = &buffer[(i-4)..i];
        if (1..4).all(|i| !window[i..].contains(&window[i - 1])) {
            println!("Part 1: {}", i);
            break;
        }
    }

    for i in 14..buffer.len() {
        let window = &buffer[(i-14)..i];
        if (1..14).all(|i| !window[i..].contains(&window[i - 1])) {
            println!("Part 2: {}", i);
            break;
        }
    }
}


fn main() {
    day06("input/input.txt");
}
