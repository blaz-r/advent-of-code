use std::fs;
use std::collections::VecDeque;

fn find_path(map: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<usize>> {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut dist: Vec<Vec<usize>> = vec![vec![0; map[0].len()]; map.len()];
    q.push_back((start.0, start.1));
    
    while q.len() > 0 {
        let (ci, cj) = q.pop_front().unwrap();
        if (ci, cj) == end {
            break;
        }

        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let ni = (ci as i32 + di) as usize;
            let nj = (cj as i32 + dj) as usize;
            if ni < map.len() && nj < map[0].len() {
                if (map[ni][nj] as i32 - map[ci][cj] as i32 <= 1) && (dist[ni][nj] == 0) {
                    q.push_back((ni, nj));
                    dist[ni][nj] = dist[ci][cj] + 1;
                }
            }
        }
    }
    dist
}


fn day12(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let mut map: Vec<Vec<u8>> = data.lines().map(|l| l.bytes().collect()).collect();

    let mut start: (usize, usize) = (0, 0);
    let mut end = (0,0);
    for (i, row) in map.clone().iter().enumerate() {
        for (j, el) in row.iter().enumerate() {
            if *el == b'S' {
                start = (i, j);
                map[i][j] = b'a';
            }
            if *el == b'E' {
                end = (i, j);
                map[i][j] = b'z';
            }
        }
    }
    let dist = find_path(&map, start, end);
    println!("Part 1: {}", dist[end.0][end.1]);

    let mut low_points: Vec<(usize, usize)> = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == b'a' {
                low_points.push((i, j));
            }
        }
    }   

    println!("Part 2: {}", low_points.iter()
                                     .map(|(i, j)| find_path(&map, (*i, *j), end)[end.0][end.1])
                                     .filter(|s| *s != 0)
                                     .min().unwrap());
}


fn main() {
    day12("input/input.txt");
}
