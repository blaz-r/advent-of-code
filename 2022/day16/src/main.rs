use std::fs;
use std::collections::HashMap;


fn set_bit(mask: &mut u64, pos: usize) {
    *mask |= 1 << pos
}


fn reset_bit(mask: &mut u64, pos: usize) {
    *mask &= !(1 << pos)
}


fn get_bit(mask: u64, pos: usize) -> bool {
    return mask & (1 << pos) > 0;
}


fn solve(map: &HashMap<&str, Vec<&str>>, flow: &HashMap<&str, i32>, valve2id: &HashMap<&str, usize>,
         memo: &mut HashMap<(i32, i32, bool, u64), i32>,
         mut open_valves: u64,
         minute: i32, pos: &str, elephant: bool) -> i32 {
    if minute < 1 {
        if elephant {
            return solve(map, flow, valve2id, memo, open_valves, 26, "AA", false);
        }
        return 0;
    }
    let i = valve2id[pos];
    match memo.get(&(i as i32, minute, elephant, open_valves)) {
        Some(n) => return *n,
        None => ()
    }
    
    let mut max = 0;
    // open current
    if !get_bit(open_valves, i) && flow[pos] != 0 {
        let release = (minute - 1) * flow[pos];
        set_bit(&mut open_valves, i);
        for valve in &map[pos] {  
            let next_pressure = solve(map, flow, valve2id, memo, open_valves, minute - 2, &valve, elephant);

            if release + next_pressure > max {
                max = release + next_pressure;
            }
        }
        reset_bit(&mut open_valves, i);
    }

    // don't open current
    for valve in &map[pos] {            
        let next_pressure = solve(map, flow, valve2id, memo, open_valves, minute - 1, &valve, elephant);

        if next_pressure > max {
            max = next_pressure;
        }
    }
    memo.insert((i as i32, minute, elephant, open_valves), max);

    max
}


fn day16(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flow_rate: HashMap<&str, i32> = HashMap::new();
    let mut valve2id: HashMap<&str, usize> = HashMap::new();

    let mut c = 0;
    for line in data.lines() {
        let split: Vec<&str> = line.split(" ").collect();

        let valve = split[1];
        let flow: i32 = split[4].split("=").nth(1).unwrap().split(";").next().unwrap().parse().unwrap();

        flow_rate.insert(valve, flow);
        valve2id.insert(valve, c);
        c += 1;
        let tunnels = &split[9..];
        let mut valve_connections: Vec<&str> = Vec::new();
        for tun in tunnels {
            let name = tun.split(",").next().unwrap();
            valve_connections.push(name);
        }
        map.insert(valve, valve_connections);
    }
    let mut memo: HashMap<(i32, i32, bool, u64), i32> = HashMap::new();
    let mut open_valves: u64 = 0;

    println!("Part 1: {}", solve(&map, &flow_rate, &valve2id, &mut memo, open_valves, 30, "AA", false));

    memo = HashMap::new();
    open_valves = 0;
    println!("Part 2: {}", solve(&map, &flow_rate, &valve2id, &mut memo, open_valves, 26, "AA", true));
}


fn main() {
    day16("input/input.txt");
}
