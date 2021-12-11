use crate::util;
use std::collections::HashSet;

fn perform_iteration(octopii: &[Vec<u8>]) -> (Vec<Vec<u8>>, usize) {
    let mut octopii = <&[std::vec::Vec<u8>]>::clone(&octopii).to_owned();
    let mut flashers: HashSet<[usize; 2]> = HashSet::new();
    let mut unprocessed_flashers: Vec<[usize; 2]> = vec![];
    // First, the energy level of each octopus increases by 1.
    for (row_id, row) in octopii.iter_mut().enumerate().take(10) {
        for (column_id, value) in row.iter_mut().enumerate().take(10) {
            *value += 1;
            if value == &10 {
                flashers.insert([row_id, column_id]);
                unprocessed_flashers.push([row_id, column_id]);
            }
        }
    }
    // Then, any octopus with an energy level greater than 9 flashes.
    // This increases the energy level of all adjacent octopuses by 1,
    // including octopuses that are diagonally adjacent.

    // If this causes an octopus to have an energy level greater than 9, it also flashes.
    // This process continues as long as new octopuses keep having their energy level
    //  increased beyond 9. (An octopus can only flash at most once per step.)
    while let Some(flasher) = unprocessed_flashers.pop() {
        // A flasher went off, increase all the ones around it!
        let [row_id, column_id] = flasher;

        for x_delta in [-1, 0, 1] {
            for y_delta in [-1, 0, 1] {
                let new_y = row_id as i32 + y_delta;
                let new_x = column_id as i32 + x_delta;
                if !(0..10).contains(&new_x) || !(0..10).contains(&new_y) {
                    continue;
                } else {
                    let x = new_x as usize;
                    let y = new_y as usize;
                    octopii[y][x] += 1;
                    if octopii[y][x] >= 10 && flashers.insert([y, x]) {
                        unprocessed_flashers.push([y, x]);
                    }
                }
            }
        }
    }

    // Finally, set octopus that flashed to 0
    for flasher in &flashers {
        octopii[flasher[0]][flasher[1]] = 0;
    }

    (octopii, flashers.len())
}

fn part_one(octopii: Vec<Vec<u8>>) {
    let mut octopii = octopii;
    let mut flashes = 0;
    for _iter in 0..100 {
        let (new_octopii, flashes_that_iter) = perform_iteration(&octopii);
        flashes += flashes_that_iter;
        octopii = new_octopii;
    }
    println!("Part One: The number of flashes was {} ", flashes);
}

fn part_two(octopii: Vec<Vec<u8>>) {
    let mut octopii = octopii;
    let mut flashes = 0;
    let mut iter = 0;
    while flashes != 100 {
        let (new_octopii, flashes_that_iter) = perform_iteration(&octopii);
        flashes = flashes_that_iter;
        octopii = new_octopii;
        iter += 1;
    }
    println!(
        "Part Two: The first time they all synchronise is on iteration {} ",
        iter
    );
}

pub(crate) fn day11() {
    // Load inputs from input directory
    let octopii: Vec<Vec<u8>> = util::load_inputs("11".to_string())
        .iter()
        .map(|x| x.chars().map(|v| v.to_digit(10).unwrap() as u8).collect())
        .collect();

    part_one(octopii.clone());
    part_two(octopii);
}
