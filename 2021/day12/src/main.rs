use std::fs;
use std::collections::{HashMap, HashSet};


fn find_paths_part1(connections: &HashMap<&str, Vec<&str>>, current: &str, visited: &mut HashSet<String>) -> i32 {
    if current == "end" {
        return 1;
    }

    if current.to_lowercase() == current {
        if visited.contains(current) {
            return 0;
        }
        else {
            visited.insert(current.to_owned());
        }
    }
    let mut found = 0;
    for adjecent in connections.get(current).unwrap().iter() {
        found += find_paths_part1(connections, adjecent, visited);
    }
    visited.remove(current);

    found
}


fn find_paths_part2(connections: &HashMap<&str, Vec<&str>>, current: &str, visited: &mut HashSet<String>, twice_visited: bool) -> i32 {
    if current == "end" {
        return 1;
    }

    let mut found = 0;
    for adjecent in connections.get(current).unwrap().iter() {
        // small
        if adjecent.to_lowercase() == *adjecent {
            // second visit
            if visited.contains(*adjecent) {
                if *adjecent != "start" && !twice_visited {
                    found += find_paths_part2(connections, adjecent, visited, true);
                }
            }
            // first visit
            else {
                if *adjecent != "start" {
                    visited.insert((*adjecent).to_owned());
                    found += find_paths_part2(connections, adjecent, visited, twice_visited);
                    visited.remove(*adjecent);
                }
            }
        }
        // large
        else {
            found += find_paths_part2(connections, adjecent, visited, twice_visited);
        }
    }

    found
}


fn day12(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let all_keys: Vec<Vec<&str>> = data.lines()
                                       .map(|line| line.split("-")
                                                       .collect()
                                            ).collect();
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::from_iter(all_keys.iter()
                                                                               .flatten()
                                                                               .map(|&vert| (vert, vec![]))
                                                                      );
    for conn in all_keys {
        connections.get_mut(conn[0]).unwrap().push(conn[1]);
        connections.get_mut(conn[1]).unwrap().push(conn[0]);
    }

    let mut visited: HashSet<String> = HashSet::new();

    let paths = find_paths_part1(&connections, "start", &mut visited);
    println!("PART1: {}", paths);

    visited.clear();
    let paths_2 = find_paths_part2(&connections, "start", &mut visited, false);
    println!("PART2: {}", paths_2);
}


fn main() {
    day12("./input/input.txt")
}
