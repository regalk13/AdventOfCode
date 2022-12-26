use std::collections::HashSet;
use itertools::Itertools;

// Defining state of current blueprint process
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
struct State {
    time_left: i32,
    ore: i32,
    clay: i32,
    obs: i32,
    geode: i32,
    ore_robot: i32,
    clay_robot: i32,
    obs_robot: i32,
    geode_robot: i32,
}

// Blueprint values
#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    idx: i32,
    ore_cost_in_ore: i32,
    clay_cost_in_ore: i32,
    obs_cost_in_ore: i32,
    obs_cost_in_clay: i32,
    geode_cost_in_ore: i32,
    geode_cost_in_obsidian: i32,
}

// First part function
fn first_part(input: &str) ->i32 {
    // Get blueprints
    let blueprints = parse(input);
    // Return the value of the function get_best and multiply by the inndex
    blueprints
        .iter()
        .map(|i| {
            i.idx
                * get_best(
                    &i,
                    State {
                        time_left: 24,
                        ore: 0,
                        clay: 0,
                        obs: 0,
                        geode: 0,
                        ore_robot: 1,
                        clay_robot: 0,
                        obs_robot: 0,
                        geode_robot: 0,
                    },
                )
        })
        .sum()
}

// Second part solution
fn second_part(input: &str) -> i32 {
    // Parsing input
    let blueprints = parse(input);
    // Take first 3 blueprints and multply by the best sequence of blueprints
    blueprints
        .iter()
        .take(3)
        .map(|b| {
            get_best(
                &b,
                State {
                    time_left: 32,
                    ore: 0,
                    clay: 0,
                    obs: 0,
                    geode: 0,
                    ore_robot: 1,
                    clay_robot: 0,
                    obs_robot: 0,
                    geode_robot: 0,
                },
            )
        })
        .product()
}

// Get the best of blue prints!
fn get_best(blueprint: &Blueprint, start_state: State) -> i32 {
    let mut scanned: HashSet<State> = HashSet::with_capacity(50 * 1024 * 1024);
    let mut to_scan = Vec::with_capacity(50 * 1024 * 1024);
    to_scan.push(start_state);

    // We use the current best state in terms of geodes
    // to discover if the evaluated state that will always be worse than the current best
    // if it is, we stop there.
    let mut best_geode = 0;
    let mut best_state = State::default();

    // As we can only build one robot a turn,
    // there is no point in producing more than what we can spend
    let max_ore = *[
        blueprint.clay_cost_in_ore,
        blueprint.ore_cost_in_ore,
        blueprint.obs_cost_in_ore,
        blueprint.geode_cost_in_ore,
    ]
    .iter()
    .max()
	.unwrap();
    
    let max_obsidian = blueprint.geode_cost_in_obsidian;
    let max_clay = blueprint.obs_cost_in_clay;

    while let Some(state) = to_scan.pop() {
        let geode_expected = state.geode + state.time_left * state.geode_robot;
        if state.time_left == 1 {
            if best_geode < geode_expected {
                best_geode = geode_expected;
                best_state = state;
            }
            continue; // end of investigation
        }

        if worse(best_state, state) {
            continue; // will always be worse than the best
        }

        if scanned.contains(&state) {
            continue; // already processed.
        }
        scanned.insert(state);

        if best_geode < geode_expected {
            best_geode = geode_expected;
            best_state = state;
        }

        to_scan.push(dig(state));
        // geode robot
        if state.ore >= blueprint.geode_cost_in_ore && state.obs >= blueprint.geode_cost_in_obsidian
        {
            let mut new_state = dig(state);
            new_state.geode_robot += 1;
            new_state.obs -= blueprint.geode_cost_in_obsidian;
            new_state.ore -= blueprint.geode_cost_in_ore;
            to_scan.push(new_state);
            continue; // seems to always be better to build a geode robot when we
        }
        // obsidian robot
        if state.ore >= blueprint.obs_cost_in_ore
            && state.clay >= blueprint.obs_cost_in_clay
            && !dont_need_more(state.time_left, state.obs, state.obs_robot, max_obsidian)
        {
            let mut new_state = dig(state);
            new_state.obs_robot += 1;
            new_state.clay -= blueprint.obs_cost_in_clay;
            new_state.ore -= blueprint.obs_cost_in_ore;
            to_scan.push(new_state);
        }
        // clay robot payoffs in >3turns
        if state.ore >= blueprint.clay_cost_in_ore
            && !dont_need_more(state.time_left, state.clay, state.clay_robot, max_clay)
            && state.time_left > 3
        {
            let mut new_state = dig(state);
            new_state.clay_robot += 1;
            new_state.ore -= blueprint.clay_cost_in_ore;
            to_scan.push(new_state);
        }
        // ore robot
        if state.ore >= blueprint.ore_cost_in_ore
            && !dont_need_more(state.time_left, state.ore, state.ore_robot, max_ore)
        {
            let mut new_state = dig(state);
            new_state.ore_robot += 1;
            new_state.ore -= blueprint.ore_cost_in_ore;
            to_scan.push(new_state);
        }
    }
    best_geode
}

// Return bool to know if we need more mineral current ask
fn dont_need_more(time_left: i32, stock: i32, robots: i32, max: i32) -> bool {
    robots >= max || time_left * robots + stock > time_left * max
}

// Return the works state between two states
fn worse(max: State, other: State) -> bool {
    (max.time_left == other.time_left
        && max.geode_robot >= other.geode_robot
        && max.obs_robot >= other.obs_robot
        && max.clay_robot >= other.clay_robot
        && max.ore_robot >= other.ore_robot
        && max.geode >= other.geode
        && max.obs >= other.obs
        && max.clay >= other.clay
        && max.ore >= other.ore)
        || other.geode
            + other.geode_robot * other.time_left
            + other.time_left * (other.time_left / 2)
            <= max.geode + max.geode_robot * max.time_left
}

// Dig into the state
fn dig(mut state: State) -> State {
    state.ore += state.ore_robot;
    state.clay += state.clay_robot;
    state.obs += state.obs_robot;
    state.geode += state.geode_robot;
    state.time_left -= 1;
    state
}

// Parsing the input, returning a vec of blueprints
fn parse(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|l| {
            let el = l.split_ascii_whitespace().collect_vec();
            Blueprint {
                idx: el[1][..el[1].len() - 1].parse::<i32>().unwrap(),
                ore_cost_in_ore: el[6].parse::<i32>().unwrap(),
                clay_cost_in_ore: el[12].parse::<i32>().unwrap(),
                obs_cost_in_ore: el[18].parse::<i32>().unwrap(),
                obs_cost_in_clay: el[21].parse::<i32>().unwrap(),
                geode_cost_in_ore: el[27].parse::<i32>().unwrap(),
                geode_cost_in_obsidian: el[30].parse::<i32>().unwrap(),
            }
        })
        .collect_vec()
}

fn main() {
    // Getting the input file
    let file = std::fs::read_to_string("./input").expect("Expected file");

    println!("Output 1: {:?}", first_part(&file));
    println!("Output 2: {:?}", second_part(&file));
}
