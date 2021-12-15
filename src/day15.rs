use crate::util;
use std::collections::{HashMap, HashSet};

fn orthogonal_points(point: &[usize; 2], risk_levels: &[Vec<u8>]) -> Vec<[usize; 2]> {
    let [row, column] = point;
    let mut orthogonal_points: Vec<[usize; 2]> = [].to_vec();
    if row != &0 {
        orthogonal_points.push([(row - 1), *column])
    }
    if row != &(risk_levels.len() - 1) {
        orthogonal_points.push([(row + 1), *column])
    }
    if column != &0 {
        orthogonal_points.push([*row, (column - 1)])
    }
    if column != &(risk_levels[0].len() - 1) {
        orthogonal_points.push([*row, (column + 1)])
    }
    orthogonal_points
}

fn find_safest_route(risk_levels: &[Vec<u8>]) -> u32 {
    // Find the cheapest way to get from top-left to bottom-right

    let mut cheapest_path_to_point: HashMap<[usize; 2], u32> = HashMap::new();
    cheapest_path_to_point.insert([0, 0], 0);

    let mut new_cheapest_path_to_point = cheapest_path_to_point.clone();
    let mut places_to_go: HashSet<[usize; 2]> = HashSet::new();
    places_to_go.insert([0, 0]);

    // Right we haven't been to all cells or the HashMap is still changing...
    while !places_to_go.is_empty() {
        cheapest_path_to_point = new_cheapest_path_to_point.clone();
        let mut new_places_to_go: HashSet<[usize; 2]> = HashSet::new();

        for (point, cost) in places_to_go
            .iter()
            .map(|p| (p, cheapest_path_to_point.get(p).unwrap()))
        {
            let orthogonal_points_to_p = orthogonal_points(point, risk_levels);

            for q in orthogonal_points_to_p {
                let path_cost_to_q = cost + risk_levels[q[0]][q[1]] as u32;
                if !new_cheapest_path_to_point.contains_key(&q)
                    || &path_cost_to_q < new_cheapest_path_to_point.get(&q).unwrap()
                {
                    new_places_to_go.insert(q);
                    new_cheapest_path_to_point.insert(q, path_cost_to_q);
                }
            }
        }
        places_to_go = new_places_to_go;
    }
    let bottom_right = [risk_levels.len() - 1, risk_levels[0].len() - 1];
    *new_cheapest_path_to_point.get(&bottom_right).unwrap()
}

fn part_one(risk_levels: &[Vec<u8>]) {
    println!("The part one answer is {}", find_safest_route(risk_levels));
}

fn part_two(risk_levels: &[Vec<u8>]) {
    let smaller_cave_height = risk_levels.len();
    let smaller_cave_width = risk_levels[0].len();
    //let mut larger_cave: Vec<Vec<u8>> = Vec::with_capacity(5*smaller_cave_height);
    let mut larger_cave: HashMap<[usize; 2], u8> = HashMap::new();

    for (row_id, row) in risk_levels.iter().enumerate() {
        for (column_id, value) in row.iter().enumerate() {
            for x in 0..5 {
                for y in 0..5 {
                    larger_cave.insert(
                        [
                            row_id + x * smaller_cave_height,
                            column_id + y * smaller_cave_width,
                        ],
                        (((value + x as u8 + y as u8) - 1) % 9) + 1,
                    );
                }
            }
        }
    }

    let mut larger_cave_as_vec: Vec<Vec<u8>> = vec![];
    for row in 0..5 * smaller_cave_height {
        let mut my_row: Vec<u8> = vec![];
        for column in 0..5 * smaller_cave_width {
            my_row.push(*larger_cave.get(&[row, column]).unwrap());
        }
        larger_cave_as_vec.push(my_row)
    }

    println!(
        "The part two answer is {}",
        find_safest_route(&larger_cave_as_vec)
    );
}

pub(crate) fn day15() {
    // Load inputs from input directory
    let risk_levels: Vec<Vec<u8>> = util::load_inputs("15".to_string())
        .iter()
        .map(|x| x.chars().map(|v| v.to_digit(10).unwrap() as u8).collect())
        .collect();

    part_one(&risk_levels);
    part_two(&risk_levels);
}
