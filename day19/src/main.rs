use std::fs;
use std::collections::{HashSet, HashMap};


fn rotate(rot: usize, v: &[i32; 3]) -> [i32; 3] {
    let [x, y, z] = *v;

    match rot {
        0 => [x, y, z],
        1 => [x, -z, y],
        2 => [x, -y, -z],
        3 => [x, z, -y],

        4 => [-x, -y, z],
        5 => [-x, -z, -y],
        6 => [-x, y, -z],
        7 => [-x, z, y],

        8 => [y, x, -z],
        9 => [y, z, x],
        10 => [y, -x, z],
        11 => [y, -z, -x],

        12 => [-y, x, z],
        13 => [-y, -z, x],
        14 => [-y, -x, -z],
        15 => [-y, z, -x],

        16 => [z, x, y],
        17 => [z, -y, x],
        18 => [z, -x, -y],
        19 => [z, y, -x],

        20 => [-z, x, -y],
        21 => [-z, y, x],
        22 => [-z, -x, y],
        23 => [-z, -y, -x],

        _ => [0, 0, 0]
    }
}


fn sub(v1: &[i32; 3], v2: &[i32; 3]) -> [i32; 3] {
    [v1[0] - v2[0], v1[1] - v2[1], v1[2] - v2[2]]
}


fn add(v1: &[i32; 3], v2: &[i32; 3]) -> [i32; 3] {
    [v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2]]
}


fn manhattan(v1: &[i32; 3], v2: &[i32; 3]) -> i32 {
    (v1[0] - v2[0]).abs() + (v1[1] - v2[1]).abs() + (v1[2] - v2[2]).abs()
}


fn find_beacons(scanners: &mut Vec<Vec<[i32; 3]>>) {
    let mut locations: Vec<[i32; 3]> = vec![[0, 0, 0]];
    let mut known: HashSet<[i32; 3]> = HashSet::new();

    for bcn in scanners.remove(0) {
        known.insert(bcn);
    }

    'outer: while scanners.len() > 0 {
        for i in 0..scanners.len() {
            for rot in 0..24{

                let mut offsets: HashMap<[i32; 3], i32> = HashMap::new();

                for known_bcn in known.iter() {
                    for curr_bcn in scanners[i].iter() {
                        let offset = sub(&known_bcn, &rotate(rot, curr_bcn));
                        *offsets.entry(offset).or_insert(0) += 1;

                        if offsets[&offset] >= 12 {
                            locations.push(offset);

                            for new_bcns in scanners.remove(i) {
                                let pos = add(&rotate(rot, &new_bcns), &offset);
                                known.insert(pos);
                            }

                            continue 'outer;
                        }
                    }
                }
            }
        }
    }

    println!("PART1: {}", known.len());

    let mut max = 0;
    for loc1 in locations.iter() {
        for loc2 in locations.iter() {
            let dist = manhattan(loc1, loc2);
            if dist > max {
                max = dist;
            }
        }
    }

    println!("PART2: {}", max);
}


fn day19(filename: &str) {
    let data = fs::read_to_string(filename).expect("can't read file");
    let scn_str: Vec<Vec<&str>> = data.split("\n\n").map(|scn| scn.lines().collect()).collect();
    let mut scanners: Vec<Vec<[i32; 3]>> = Vec::new();

    for scn in scn_str {
        let mut scanner: Vec<[i32; 3]> = Vec::new();
        for line in &scn[1..] {
            let coords: Vec<i32> = line.split(",").map(|num| num.parse::<i32>().unwrap()).collect();
            scanner.push([coords[0], coords[1], coords[2]]);
        }
        scanners.push(scanner);
    }

    find_beacons(&mut scanners);
}


fn main() {
    day19("./input/input.txt")
}
