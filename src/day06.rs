use std::collections::HashMap;

use crate::util;

fn part_one(mut lanternfish: Vec<i8>) {
    for _day in 0..80 {
        for (pos, fish) in lanternfish.clone().iter().enumerate() {
            match fish {
                0 => {
                    lanternfish[pos] = 6;
                    lanternfish.push(8);
                }
                _ => {
                    lanternfish[pos] = fish - 1;
                }
            }
        }
    }
    println!(
        "After 80 days there will be {} lanternship",
        lanternfish.len()
    );
}

fn part_two(lanternfish: Vec<i8>) {
    let mut lanternfish_hash: HashMap<i8, i64> = HashMap::new();
    for f in 0..7 {
        lanternfish_hash.insert(f, 0);
    }
    for f in lanternfish.iter() {
        let counter = lanternfish_hash.entry(*f).or_insert(0);
        *counter += 1;
    }
    for _day in 0..256 {
        let clone_lf = lanternfish_hash.clone();
        for (key, val) in clone_lf.iter() {
            match key {
                0 => {
                    lanternfish_hash.insert(8, *val);
                }
                _ => {
                    lanternfish_hash.insert(key - 1, *val);
                }
            }
        }
        lanternfish_hash.insert(
            6,
            clone_lf.get(&0).unwrap_or(&0) + clone_lf.get(&7).unwrap_or(&0),
        );
    }
    println!(
        "After 256 days there will be {} lanternship",
        lanternfish_hash.values().copied().sum::<i64>()
    );
}

pub(crate) fn day06() {
    // Load inputs from input directory
    let lanternfish: Vec<i8> = util::load_inputs("06".to_string())
        .get(0)
        .unwrap()
        .split(',')
        .map(|v| v.parse::<i8>().unwrap())
        .collect();

    part_one(lanternfish.clone());
    part_two(lanternfish);
}
