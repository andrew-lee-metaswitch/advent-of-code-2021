use crate::util;
use regex::Regex;
use std::cmp;
use std::cmp::Ordering;
//use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Probe {
    initial_x_velocity: i32,
    initial_y_velocity: i32,
}

impl Probe {
    fn max_height(&self) -> i32 {
        let mut current_y_velocity = self.initial_y_velocity;
        let mut current_y_position = 0;
        while current_y_velocity >= 0 {
            current_y_position += current_y_velocity;
            current_y_velocity -= 1;
        }
        current_y_position
    }

    fn probe_lands_in_box(&self, x: [i32; 2], y: [i32; 2]) -> bool {
        let mut current_position = [0, 0];
        let mut current_x_velocity = self.initial_x_velocity;
        let mut current_y_velocity = self.initial_y_velocity;

        let x_box = std::ops::Range {
            start: x[0],
            end: x[1] + 1,
        };
        let y_box = std::ops::Range {
            start: y[0],
            end: y[1] + 1,
        };

        while !x_box.contains(&current_position[0]) || !y_box.contains(&current_position[1]) {
            // println!("Currnet position is {:?}", current_position);
            if current_x_velocity <= 0 && current_position[0] < x[0]
                || current_x_velocity >= 0 && current_position[0] > x[1]
                || current_y_velocity < 0 && current_position[1] < y[0]
            {
                // We're heading left of the box and are already left of it, or we're heading right of the box and are already left of it
                return false;
            };

            current_position = [
                current_position[0] + current_x_velocity,
                current_position[1] + current_y_velocity,
            ];
            match current_x_velocity.cmp(&0) {
                Ordering::Greater => current_x_velocity -= 1,
                Ordering::Less => current_x_velocity += 1,
                Ordering::Equal => {}
            };

            current_y_velocity -= 1;
        }
        // println!("current_position a {:?}", current_position);
        true
    }
}

fn part_one_and_two(x: [i32; 2], y: [i32; 2]) {
    println!("The target box is {:?}, {:?}", x, y);

    // Let x' = initial_x_velocity. Clearly to have a hope to land in the box x' > 1, <= x[1]
    // (that x' can't be negative, and it can't be too powerful)

    // in x direction, we will stop moving in x' turns (as after then current-x-velocty will be 0)
    // in that time we will have travelled (x')^2 - t(x') in the x-direction, where t(a) is the ath triangle number.
    // By maths, (x')^2 - t(x') = t(x'-1) > x[0]
    // By maths a < sqrt(x[0]) => t(a) < x[0], so x' > sqrt(x[0]) + 1
    let x_min = ((x[0] as f32).sqrt().floor() as i32) + 1;

    // y will have velocity  y' + y' -1 + y' -2 = ny' - t(n-1).

    // for y velocity n>0, note after 2n+1 iterations it will have gone
    // n + n-1   + 2 + 1 + 0 -1 -2   -3    -n step ups up, i.e. will be at net-0 in y-axis, then after 2n+2 iterations it will be at -n.
    // So n < y[0].abs(), else probe will miss the y-box
    // Furthermore y_velcoty > y[0], else we will go below box immediately,

    let mut max_y_height = 0;
    let mut successful_probes = 0;

    for initial_y_velocity in y[0]..y[0].abs() {
        for initial_x_velocity in x_min..x[1] + 1 {
            let probe = Probe {
                initial_x_velocity,
                initial_y_velocity,
            };
            if probe.probe_lands_in_box(x, y) {
                println!("Probe {:?} lands in box! x={:?}, y= {:?} ", probe, x, y);
                max_y_height = cmp::max(max_y_height, probe.max_height());
                successful_probes += 1;
            }
        }
    }
    println!("The max height is  {} ", max_y_height);
    println!("The number of succesful probes is  {} ", successful_probes);
}

pub(crate) fn day17() {
    // Load inputs from input directory
    let input_str = util::load_inputs("17".to_string())
        .get(0)
        .unwrap()
        .to_string();

    let re = Regex::new(r#"target area: x=(\d+)..(\d+), y=(-\d+)..(-\d+)"#).unwrap();
    if let Some(cap) = re.captures_iter(&input_str).next() {
        // Captures({0: Some("781,721 -> 781,611"), 1: Some("781"), 2: Some("721"), 3: Some("781"), 4: Some("611")})
        let x_range: [i32; 2] = [
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
        ];
        let y_range: [i32; 2] = [
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        ];
        println!("Yis {:?}", y_range);
        part_one_and_two(x_range, y_range);
    };
}
