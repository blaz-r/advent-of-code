use std::fs;
use std::cmp;

fn read_packet(bits: &Vec<u8>, mut index: usize) -> (usize, u32, u64) {
        let mut version: u32 = 0;
        for _ in index..index+3 {
            version *= 2;
            version += bits[index] as u32;
            index += 1;
        }
        let mut pack_type = 0;
        for _ in index..index+3 {
            pack_type *= 2;
            pack_type += bits[index];
            index += 1;
        }

        let mut value: u64 = 0;
        
        let mut sum: u64 = 0;
        let mut product: u64 = 1;
        let mut min: u64 = u64::MAX;
        let mut max: u64 = 0;
        let mut comp: i64 = 0;

        // number packet
        if pack_type == 4 {
            loop {
                let last = bits[index];
                index += 1;
                for _ in 0..4 {
                    value *= 2;
                    value += bits[index] as u64;
                    index += 1;
                }

                if last != 1 {
                    break;
                }
            }
        }
        // operator
        else {
            let len_type = bits[index];
            index += 1;
            // 15 bits -> len of sub-packets
            if len_type == 0 {
                let mut len_of_bits: u16 = 0;
                for _ in 0..15 {
                    len_of_bits *= 2;
                    len_of_bits += bits[index] as u16;
                    index += 1;
                }
                let mut done_len = 0;
                loop {
                    let (r_ind, r_ver, r_val) = read_packet(bits, index);
                    done_len += r_ind - index;
                    index = r_ind;

                    // operator based on type
                    match pack_type {
                        0 => sum += r_val,
                        1 => product *= r_val,
                        2 => min = cmp::min(min, r_val),
                        3 => max = cmp::max(max, r_val),
                        5 | 6 | 7 => { if comp == 0 {
                                            comp = r_val as i64;
                                        } else {
                                            comp = comp - r_val as i64;
                                            match pack_type {
                                                5 => if comp > 0 { comp = 1; } else { comp = 0; }, 
                                                6 => if comp < 0 { comp = 1; } else { comp = 0; }, 
                                                7 => if comp == 0 { comp = 1; } else { comp = 0; }, 
                                                _ => ()
                                            }
                                        }},
                        _ => ()
                    }

                    // part 1
                    version += r_ver;

                    if done_len == len_of_bits as usize {
                        break;
                    }
                }
            }
            // next 11 -> number of subpackets
            else {
                let mut num_of_packets: u16 = 0;
                for _ in 0..11 {
                    num_of_packets *= 2;
                    num_of_packets += bits[index] as u16;
                    index += 1;
                }
                let mut done_num = 0;
                loop {
                    let (r_ind, r_ver, r_val) = read_packet(bits, index);
                    index = r_ind;
                    done_num += 1;

                    // operator based on type
                    match pack_type {
                        0 => sum += r_val,
                        1 => product *= r_val,
                        2 => min = cmp::min(min, r_val),
                        3 => max = cmp::max(max, r_val),
                        5 | 6 | 7 => { if comp == 0 {
                                            comp = r_val as i64;
                                        } else {
                                            comp = comp - r_val as i64;
                                            match pack_type {
                                                5 => if comp > 0 { comp = 1; } else { comp = 0; }, 
                                                6 => if comp < 0 { comp = 1; } else { comp = 0; }, 
                                                7 => if comp == 0 { comp = 1; } else { comp = 0; }, 
                                                _ => ()
                                            }
                                        }},
                        _ => ()
                    }

                    // part 1
                    version += r_ver;

                    if done_num == num_of_packets {
                        break;
                    }
                }
            }
        }

        // final return value based on type
        match pack_type {
            0 => value = sum,
            1 => value = product,
            2 => value = min,
            3 => value = max,
            5 | 6 | 7 => value = comp as u64,
            _ => () // value is a number
        }

        (index, version, value)
}


fn day16(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let bits: Vec<u8> = data.chars().map(|c| 
                                            { let num = c.to_digit(16).unwrap();
                                                (0..4).map(|i| (num >> (3 - i) & 0x1) as u8).collect::<Vec<u8>>()
                                            }
                                        ).flatten()
                                        .collect();

    let (_, ver_sum, value) = read_packet(&bits, 0);
    println!("PART1: {}", ver_sum);
    println!("PART2: {}", value);
}


fn main() {
    day16("./input/input.txt");
}
