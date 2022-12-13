use std::fs;
use std::cmp::Ordering::{Less, Equal, Greater};
use Packet::{List, Num};


#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Num(i32),
    List(Vec<Packet>)
}

impl Packet {
    fn create(str: &str) -> Packet {
        let packet: Packet;
        match str.chars().next().unwrap() {
            '[' => {
                let mut packet_list: Vec<Packet> = Vec::new();
                let mut s = 1;
                let mut depth = 0;

                for (i, c) in str.chars().enumerate().skip(1) {
                    match c {
                        '[' => depth += 1,
                        ']' => {
                            if depth > 0 {
                                depth -= 1;
                            }
                            else if s < i {
                                packet_list.push(Packet::create(&str[s..i]));
                                s = i + 1;
                            }
                        }
                        ',' => {
                            if depth == 0 {
                                packet_list.push(Packet::create(&str[s..i]));
                                s = i + 1;
                            }
                        }
                        _ => continue
                    }
                }
                packet = Packet::List(packet_list);
            }
            _ => packet = Packet::Num(str.parse().unwrap())
        }
        packet
    }

    fn num_to_list(&self) -> Packet {
        return Packet::List(vec![self.clone()]);
    }
}


impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Num(_), Packet::List(_)) => self.num_to_list().cmp(other),
            (Packet::List(_), Packet::Num(_)) => self.cmp(&other.num_to_list()),
            (Packet::Num(n1), Packet::Num(n2)) => n1.cmp(n2),
            (Packet::List(l1), Packet::List(l2)) => {
                let min_size = usize::min(l1.len(), l2.len());
                for i in 0..min_size {
                    match l1[i].cmp(&l2[i]) {
                        Less => return Less,
                        Equal => continue,
                        Greater => return Greater
                    }
                }
                return l1.len().cmp(&l2.len());
            }
        }
    }
}


fn day13(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let packets: Vec<(Packet, Packet)> = data.split("\n\n").map(|p_pair| 
                                                                    p_pair.lines()
                                                                          .map(|packet| Packet::create(packet)
                                                                              ).collect()
                                                                ).map(|pair: Vec<Packet>| (pair[0].clone(), pair[1].clone())).collect();
    let sum_1: usize = packets.iter()
                              .enumerate()
                              .filter(|(_, (p1, p2))| p1.le(&p2))
                              .map(|(i, _)| i + 1)
                              .sum();

    println!("Part 1: {}", sum_1);

    let mut all_packets: Vec<Packet> = packets.iter()
                                          .map(|(p1, p2)| vec![p1.clone(), p2.clone()])
                                          .collect::<Vec<Vec<Packet>>>()
                                          .into_iter()
                                          .flatten()
                                          .collect();

    let marker1 = List(vec![List(vec![Num(2)])]);
    let marker2 = List(vec![List(vec![Num(6)])]);
    all_packets.push(marker1.clone());
    all_packets.push(marker2.clone());

    all_packets.sort();
    let i1 = all_packets.iter().position(|p| p.clone() == marker1).unwrap() + 1;
    let i2 = all_packets.iter().position(|p| p.clone() == marker2).unwrap() + 1;

    println!("Part 2: {}", i1 * i2);
}


fn main() {
    day13("input/input.txt");
}
