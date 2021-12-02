use crate::util;

fn part_one(sea_depth_measurements: &[u16]) {
    let mut times_it_gets_deeper: usize = 0;

    for (s_position, s) in sea_depth_measurements.iter().enumerate() {
        if s_position < sea_depth_measurements.len() - 1
            && sea_depth_measurements[s_position + 1] > *s
        {
            times_it_gets_deeper += 1;
        }
    }
    println!("The times is gets deeper is {}", times_it_gets_deeper);
}

fn part_two(sea_depth_measurements: &[u16]) {
    // This really boils down to comapring the measurement 3 along from where you are.
    let mut times_it_gets_deeper: usize = 0;

    for (s_position, s) in sea_depth_measurements.iter().enumerate() {
        if s_position < sea_depth_measurements.len() - 3
            && sea_depth_measurements[s_position + 3] > *s
        {
            times_it_gets_deeper += 1;
        }
    }

    println!("The times is gets deeper is {}", times_it_gets_deeper);
}

pub(crate) fn day01() {
    // Load inputs from input directory
    let sea_depth_measurements: Vec<u16> = util::load_inputs("01".to_string())
        .iter()
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    part_one(&sea_depth_measurements);
    part_two(&sea_depth_measurements)
}
