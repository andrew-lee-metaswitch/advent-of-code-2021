use crate::util;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::iter::FromIterator;

fn part_one(submarine_diags: &[Vec<u32>]) {
    let mut gamma_rate_vec: Vec<char> = [].to_vec();
    let mut epsilon_rate_vec: Vec<char> = [].to_vec();
    for bit in 0..submarine_diags[0].len() {
        let mut bit_sum = 0;

        for diag_reading in submarine_diags {
            bit_sum += diag_reading[bit];
        }
        if usize::try_from(bit_sum).unwrap() > submarine_diags.len() / 2 {
            gamma_rate_vec.push('1');
            epsilon_rate_vec.push('0');
        } else {
            gamma_rate_vec.push('0');
            epsilon_rate_vec.push('1');
        }
    }
    let gamma_rate = u64::from_str_radix(&String::from_iter(gamma_rate_vec), 2).unwrap();
    let epsilon_rate = u64::from_str_radix(&String::from_iter(epsilon_rate_vec), 2).unwrap();

    println!("The Power Consumption is {}", gamma_rate * epsilon_rate);
}

fn most_popular(submarine_diags: &[Vec<u32>], bit: usize) -> u32 {
    let mut bit_sum = 0;

    for diag_reading in submarine_diags {
        bit_sum += diag_reading[bit];
    }

    match usize::try_from(2 * bit_sum)
        .unwrap()
        .cmp(&submarine_diags.len())
    {
        Ordering::Greater => 1,
        Ordering::Less => 0,
        Ordering::Equal => 2,
    }
}

fn part_two(submarine_diags: &[Vec<u32>]) {

    // Create a mutable vector to each
    let mut oxygen_gen_diags = submarine_diags.clone().to_owned();
    let mut co2_srubber_diags = submarine_diags.clone().to_owned();
    let mut bit = 0;

    // Loop over the mutable oxygen_gen_diags vector until only one reading is left
    while oxygen_gen_diags.len() > 1 {

        // Find which is the most common digit in the 'bit' bit
        match most_popular(&oxygen_gen_diags, bit) {
            // If 0 is the most popular digit, filter in only the diag-lines with '0' in that digit
            0 => {
                oxygen_gen_diags = oxygen_gen_diags
                    .iter()
                    .filter(|v| v[bit] == 0)
                    .map(|v| v.to_owned())
                    .collect::<Vec<Vec<u32>>>()
            }
             // If 1 is the most popular digit, or it is a tie, filter in only the diag-lines with '1' in that digit
            _ => {
                oxygen_gen_diags = oxygen_gen_diags
                    .iter()
                    .filter(|v| v[bit] == 1)
                    .map(|v| v.to_owned())
                    .collect::<Vec<Vec<u32>>>()
            }
        }
        // Increase the bit count
        bit += 1;
    }
    bit = 0;
    while co2_srubber_diags.len() > 1 {
        match most_popular(&co2_srubber_diags, bit) {
            0 => {
                co2_srubber_diags = co2_srubber_diags
                    .iter()
                    .filter(|v| v[bit] == 1)
                    .map(|v| v.to_owned())
                    .collect::<Vec<Vec<u32>>>()
            }
            _ => {
                co2_srubber_diags = co2_srubber_diags
                    .iter()
                    .filter(|v| v[bit] == 0)
                    .map(|v| v.to_owned())
                    .collect::<Vec<Vec<u32>>>()
            }
        }
        bit += 1
    }
    let oxygen_gen_rating = u64::from_str_radix(
        &oxygen_gen_diags[0]
            .iter()
            .map(|v| *v as u8)
            .map(|v| v.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();
    let co2_scrubber_rating = u64::from_str_radix(
        &co2_srubber_diags[0]
            .iter()
            .map(|v| *v as u8)
            .map(|v| v.to_string())
            .collect::<String>(),
        2,
    )
    .unwrap();

    println!(
        "The Life Rating is {}",
        oxygen_gen_rating * co2_scrubber_rating
    );
}

pub(crate) fn day03() {
    // Load inputs from input directory
    let submarine_diags: Vec<Vec<u32>> = util::load_inputs("03".to_string())
        .iter()
        .map(|v| v.chars().map(|v| v.to_digit(2).unwrap()).collect())
        .collect();

    part_one(&submarine_diags);
    part_two(&submarine_diags);
}
