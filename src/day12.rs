use itertools::Itertools;

use crate::util;
use std::fmt;
use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
enum CaveType {
    Start,
    End,
    BigCave,
    SmallCave,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Cave {
    name: String,
    cave_type: CaveType,
}

impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl FromStr for Cave {
    type Err = regex::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cave_type = match input {
            "start" => CaveType::Start,
            "end" => CaveType::End,
            _ => {
                if input == input.to_uppercase().as_str() {
                    CaveType::BigCave
                } else {
                    CaveType::SmallCave
                }
            }
        };
        Ok(Cave {
            name: input.to_string(),
            cave_type,
        })
    }
}

fn go_cave_exploring(cave_graph: &[[Cave; 2]], can_visit_a_small_cave_twice: bool) -> usize {
    let caves: Vec<Cave> = cave_graph
        .iter()
        .map(|[_u, v]| v)
        .chain(cave_graph.iter().map(|[u, _v]| u))
        .cloned()
        .collect();
    let mut get_connected_caves_map: HashMap<&Cave, Vec<Cave>> = HashMap::new();
    for c in caves.iter() {
        get_connected_caves_map.insert(c, get_connected_caves(c, cave_graph));
    }

    let mut routes_count: usize = 0;
    let mut incomplete_routes: Vec<Vec<Cave>> = vec![vec![Cave {
        name: "start".to_string(),
        cave_type: CaveType::Start,
    }]];

    while let Some(incomplete_route) = incomplete_routes.pop() {
        // We have an incomplete route, [start, a, b, C] say,
        // Let's find all the connected caves to C, say b, e, f, G, say,
        // and append [start, a, b, C, e], [start, a, b, C, G], back to incomplete routes,
        // and append [start, a, b, C, end] to the complete routes
        let next_cave_can_repeat_small_cave = can_visit_a_small_cave_twice && {
            incomplete_route
                .iter()
                .filter(|v| v.cave_type == CaveType::SmallCave)
                .unique()
                .count()
                == incomplete_route
                    .iter()
                    .filter(|v| v.cave_type == CaveType::SmallCave)
                    .count()
        };
        let last_visited_cave = incomplete_route.last().unwrap();
        for unexplored_direction in get_connected_caves_map.get(&last_visited_cave).unwrap() {
            if unexplored_direction.cave_type == CaveType::Start
                || unexplored_direction.cave_type == CaveType::SmallCave
                    && incomplete_route.contains(unexplored_direction)
                    && !next_cave_can_repeat_small_cave
            {
                // Can't go back to a start twice
                // Can't go back to a small cave more than 'max_visits_to_small_caves' times
                continue;
            } else if unexplored_direction.cave_type == CaveType::End {
                routes_count += 1;
            } else {
                let extended_route = {
                    let mut new_route = incomplete_route.clone();
                    new_route.push(unexplored_direction.clone());
                    new_route
                };
                incomplete_routes.push(extended_route);
            }
        }
    }

    routes_count
}

fn get_connected_caves(cave: &Cave, cave_graph: &[[Cave; 2]]) -> Vec<Cave> {
    cave_graph
        .iter()
        .filter(|[u, _v]| u == cave)
        .map(|[_u, v]| v)
        .chain(
            cave_graph
                .iter()
                .filter(|[_u, v]| v == cave)
                .map(|[u, _v]| u),
        )
        .cloned()
        .collect()
}

fn part_one(cave_graph: Vec<[Cave; 2]>) {
    println!(
        "The answer to part one is {} distinct routes",
        go_cave_exploring(&cave_graph, false)
    );
}

fn part_two(cave_graph: Vec<[Cave; 2]>) {
    println!(
        "The answer to part two is {} distinct routes",
        go_cave_exploring(&cave_graph, true)
    );
}

pub(crate) fn day12() {
    // Load inputs from input directory
    let cave_graph: Vec<[Cave; 2]> = util::load_inputs("12".to_string())
        .iter()
        .map(|x| {
            let mut caves = x.split('-');
            [
                Cave::from_str(caves.next().unwrap()).unwrap(),
                Cave::from_str(caves.next().unwrap()).unwrap(),
            ]
        })
        .collect();

    part_one(cave_graph.clone());
    part_two(cave_graph);
}
