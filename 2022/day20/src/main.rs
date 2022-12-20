use std::fs;


fn mix(enc_enum: &mut Vec<(usize, i64)>) {
    for i in 0..enc_enum.len() {
        let index: usize = enc_enum.iter()
                                   .position(|(order_i, _)| *order_i == i)
                                   .unwrap();

        let (ord_i, val) = enc_enum.remove(index);
        let l = enc_enum.len() as i64;
        let mut new_index = (index as i64 + val) % l;
        if new_index <= 0 {
            new_index = (l + new_index) % (l+1);
        }
        enc_enum.insert(new_index as usize, (ord_i, val));
    }
}


fn get_coord_sum(enc_enum: &Vec<(usize, i64)>) -> i64 {
    let zero_index: usize = enc_enum.iter()
                                    .position(|(_, e)| *e == 0)
                                    .unwrap();
    let sum: i64 = [1000, 2000, 3000].iter()
                                     .map(|n| enc_enum[(zero_index + n) % enc_enum.len()].1)
                                     .sum();
    sum
}


fn day20(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let encrypted: Vec<i64> = data.lines()
                                  .map(|l| l.parse()
                                                  .unwrap()
                                      ).collect();
    let mut enc_enum: Vec<(usize, i64)> = encrypted.iter()
                                                   .enumerate()
                                                   .map(|(i, e)| (i, *e))
                                                   .collect();
                                                   
    let mut enc_enum_2 = enc_enum.clone();
    mix(&mut enc_enum);
    println!("Part 1: {}", get_coord_sum(&enc_enum));

    enc_enum_2.iter_mut().for_each(|(_, e)| *e *= 811589153);
    for _ in 0..10 {
        mix(&mut enc_enum_2);
    }
    println!("Part 2: {}", get_coord_sum(&enc_enum_2));
}


fn main() {
    day20("input/input.txt");
}
