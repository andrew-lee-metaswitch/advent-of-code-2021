use crate::util;

fn part_one(crab_submarines: &[i32]) {
    let mut distances: Vec<i32> = [].to_vec();
    for position in
        *crab_submarines.iter().min().unwrap()..*crab_submarines.iter().max().unwrap() + 1
    {
        distances.push(
            crab_submarines
                .iter()
                .map(|v| (v - position).abs())
                .sum::<i32>(),
        );
    }
    println!(
        "The minimum amount of fuel to align is {:?}",
        distances.iter().min().unwrap()
    );
}

fn part_two(crab_submarines: &[i32]) {
    let mut distances: Vec<i32> = [].to_vec();
    for position in
        *crab_submarines.iter().min().unwrap()..*crab_submarines.iter().max().unwrap() + 1
    {
        distances.push(
            crab_submarines
                .iter()
                .map(|v| (v - position).abs())
                .map(|v| (v * (v + 1)) / 2)
                .sum::<i32>(),
        );
    }
    println!(
        "The minimum amount of fuel to align is {:?}",
        distances.iter().min().unwrap()
    );
}

pub(crate) fn day07() {
    // Load inputs from input directory
    let crab_submarines: Vec<i32> = util::load_inputs("07".to_string())
        .get(0)
        .unwrap()
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    part_one(&crab_submarines);
    part_two(&crab_submarines);
}
