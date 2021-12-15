use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize)
}


impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}


impl PartialOrd for State {
    fn partial_cmp(&self, other:&Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


// dijkstra
fn shortest_path(map: &Vec<Vec<i32>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut heap = BinaryHeap::new();
    let mut dist: Vec<Vec<usize>> = vec![vec![usize::MAX; map[0].len()]; map.len()];

    dist[start.0][start.1] = 0;
    heap.push(State { cost: 0, position: start});

    while let Some(State { cost, position }) = heap.pop() {
        if position == end { return cost; }

        let dirs = [(0, -1), (-1, 0), (0, 1), (1, 0)];
        for mov in dirs {
            let next_pos = (position.0 as i32 + mov.0, position.1 as i32 + mov.1);
            if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 as usize == map.len() || next_pos.1 as usize == map[0].len() {
                continue;
            }
            let next = State { cost: cost + map[next_pos.0 as usize][next_pos.1 as usize] as usize, position: (next_pos.0 as usize, next_pos.1 as usize) };

            if next.cost < dist[next.position.0][next.position.1] {
                heap.push(next);
                dist[next.position.0][next.position.1] = next.cost;
            }
        }
    }

    0
}

fn day15(filename: &str) {
    let data = fs::read_to_string(filename).unwrap();
    let map: Vec<Vec<i32>> = data.lines()
                                 .map(|line| line.chars()
                                                 .map(|c| c.to_digit(10).unwrap() as i32)
                                                 .collect::<Vec<i32>>()
                                     ).collect();

   println!("PART1: {}", shortest_path(&map, (0, 0), (map.len()-1, map[0].len()-1)));

   // part 2 map is just 1st in a 5x5 grid but every left and down increases by 1 and from 9 warps back to 1
   let h = map.len();
   let w = map[0].len();
   let mut big_map: Vec<Vec<i32>> = vec![vec![0; map[0].len()*5]; map.len()*5];
   for i in 0..5 {
       for j in 0..5 {
           for k in 0..map.len() {
               for l in 0..map[k].len() {
                   // modulo (x - 1) % (y - 1) + 1 -> modulo on inteerval [1, y-1]
                   big_map[h*i + k][w*j + l] = ((map[k][l] as usize + i*1 + j*1 - 1) % 9 + 1) as i32;
               }
           }
       }
   }

   println!("PART2: {}", shortest_path(&big_map, (0, 0), (big_map.len()-1, big_map[0].len()-1)));
}


fn main() {
    day15("./input/input.txt");
}
