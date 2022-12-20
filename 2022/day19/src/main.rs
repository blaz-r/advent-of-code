use regex::Regex;
use std::{fs, collections::HashMap};
use rayon::prelude::*;

#[derive(Debug)]
struct Blueprint {
    ore_ore_cost: i32,
    clay_ore_cost: i32,
    obs_ore_cost: i32,
    obs_clay_cost: i32,
    geode_ore_cost: i32,
    geode_obs_cost: i32
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
struct State {
    time: i32,
    ore: i32,
    clay: i32,
    obs: i32,
    geode: i32,
    ore_miners: i32,
    clay_miners: i32,
    obs_miners: i32,
    geode_miners: i32
}


fn solve(blueprint: &Blueprint, state: State, seen: &mut HashMap<State, i32>) -> i32 {
    let &Blueprint {
        ore_ore_cost,
        clay_ore_cost,
        obs_ore_cost,
        obs_clay_cost,
        geode_ore_cost,
        geode_obs_cost,
    } = blueprint;

    let mut best = 0;
    let max_ore_cost = ore_ore_cost.max(clay_ore_cost).max(obs_ore_cost).max(geode_ore_cost);

    if seen.contains_key(&state) {
        return seen[&state];
    }
    if state.time == 0 {
        return state.geode;
    }

    let mut new_state = state;
    new_state.ore += state.ore_miners;
    new_state.clay += state.clay_miners;
    new_state.obs += state.obs_miners;
    new_state.geode += state.geode_miners;

    new_state.time -= 1;

    if state.ore >= geode_ore_cost && state.obs >= geode_obs_cost {
        let mut geode_state = new_state;
        geode_state.ore -= geode_ore_cost;
        geode_state.obs -= geode_obs_cost;
        geode_state.geode_miners += 1;

        return solve(blueprint, geode_state, seen);
    } 

    if state.ore >= obs_ore_cost && state.clay >= obs_clay_cost &&
        new_state.obs_miners < geode_obs_cost {
        let mut obs_state = new_state;
        obs_state.ore -= obs_ore_cost;
        obs_state.clay -= obs_clay_cost;
        obs_state.obs_miners += 1;

        let obs_best = solve(blueprint, obs_state, seen);

        best = best.max(obs_best);
    }

    let mut used_ore = false;

    if state.ore >= clay_ore_cost && state.clay_miners < obs_clay_cost  {
        let mut clay_state = new_state;
        clay_state.ore -= clay_ore_cost;
        clay_state.clay_miners += 1;

        used_ore = true;

        let clay_best = solve(blueprint, clay_state, seen);

        best = best.max(clay_best);
    }

    if state.ore >= ore_ore_cost && state.ore_miners < max_ore_cost {
        let mut ore_state = new_state;
        ore_state.ore -= ore_ore_cost;
        ore_state.ore_miners += 1;

        used_ore = true;

        let ore_best = solve(blueprint, ore_state, seen);

        best = best.max(ore_best);
    }

    if !used_ore || new_state.ore <= max_ore_cost * state.time {
        let no_robot_best = solve(blueprint, new_state, seen);
        best = best.max(no_robot_best);
    }

    seen.insert(state, best);

    best
}


fn day19(file_name: &str) {
    let data = fs::read_to_string(file_name).expect("Can't read file");
    let num_regex = Regex::new(r"\d+").unwrap();
    let blueprints: Vec<Blueprint> = data.lines()
                                         .map(|line| {
                                                let numbers: Vec<i32> = num_regex.find_iter(line)
                                                                                .map(|n| n.as_str()
                                                                                                .parse::<i32>()
                                                                                                .unwrap()
                                                                                    ).collect();
                                                Blueprint {
                                                    ore_ore_cost: numbers[1],
                                                    clay_ore_cost: numbers[2],
                                                    obs_ore_cost: numbers[3],
                                                    obs_clay_cost: numbers[4],
                                                    geode_ore_cost: numbers[5],
                                                    geode_obs_cost: numbers[6]
                                                }
                                              }).collect();
                                              
    let initial_state = State{time: 24, ore_miners: 1, ..Default::default()};
    
    let sum_1: i32 = blueprints.par_iter()
                               .enumerate()
                               .map(|(i, b)| (i + 1) as i32 * solve(b, initial_state, &mut HashMap::new())
                                    ).sum();
    println!("Part 1: {}", sum_1);

    let initial_state_2 = State{time: 32, ore_miners: 1, ..Default::default()};
    let sum_2: i32 = blueprints[..3].par_iter()
                                    .map(|b| solve(b, initial_state_2, &mut HashMap::new())
                                        ).product();
    println!("Part 2: {}", sum_2);
}


fn main() {
    day19("input/input.txt");
}
