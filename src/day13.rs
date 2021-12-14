use itertools::Itertools;
use regex::Regex;

use crate::util;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
enum FoldDirection {
    HorizontalAxis,
    VerticalAxis,
}

fn perform_fold(p: [i32; 2], fold: &(FoldDirection, i32)) -> [i32; 2] {
    match fold.0 {
        FoldDirection::HorizontalAxis => {
            // Let p = (6,10), fold = (x, 7)
            // (6, 10) represents 6th column, 10th row
            // so 6 stays the same, but we move from 10th row to 10-2*(10-7)
            if p[1] <= fold.1 {
                p
            } else {
                [p[0], p[1] - 2 * (p[1] - fold.1)]
            }
        }
        FoldDirection::VerticalAxis => {
            // Let p = (6,10), fold = (x, 7)
            // (6, 10) represents 6th column, 10th row
            // so 6 stays the same, but we move from 10th row to 10-2*(10-7)
            if p[0] <= fold.1 {
                p
            } else {
                [p[0] - 2 * (p[0] - fold.1), p[1]]
            }
        }
    }
}

fn part_one(dots: Vec<[i32; 2]>, fold: &(FoldDirection, i32)) {
    println!(
        "The answer to part one is {}",
        dots.iter().map(|p| perform_fold(*p, fold)).unique().count()
    );
}

fn part_two(mut dots: Vec<[i32; 2]>, folds: Vec<(FoldDirection, i32)>) {
    for fold in folds.iter() {
        dots = dots
            .iter()
            .map(|p| perform_fold(*p, fold))
            .unique()
            .collect();
    }

    for x in 0..6 {
        for y in 0..40 {
            if dots.contains(&[y, x]) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

pub(crate) fn day13() {
    // Load inputs from input directory
    let mut dots: Vec<[i32; 2]> = vec![];
    let mut folds: Vec<(FoldDirection, i32)> = vec![];

    for line in util::load_inputs("13".to_string()).iter() {
        if line.contains(',') {
            let mut point_coords = line.split(',');
            dots.push([
                point_coords.next().unwrap().parse::<i32>().unwrap(),
                point_coords.next().unwrap().parse::<i32>().unwrap(),
            ]);
        } else if line.contains('=') {
            let re = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
            if let Some(cap) = re.captures_iter(line).next() {
                let fold_direction = match &cap[1] {
                    "x" => FoldDirection::VerticalAxis,
                    "y" => FoldDirection::HorizontalAxis,
                    _ => panic!("Whoop-si-daisy"),
                };
                let coordinate = cap[2].parse::<i32>().unwrap();
                folds.push((fold_direction, coordinate));
            };
        }
    }

    part_one(dots.clone(), folds.first().unwrap());
    part_two(dots, folds);
}
