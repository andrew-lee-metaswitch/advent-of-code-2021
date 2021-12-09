use crate::util;
use itertools::Itertools;
use regex::{Match, Regex};
use std::{collections::HashMap, convert::TryInto, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
struct SevenSegementDisplay {
    left: [String; 10],
    right: [String; 4],
}

const ATLAS: [&str; 10] = [
    // the zero // one etc..
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

impl FromStr for SevenSegementDisplay {
    type Err = regex::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\w+)").unwrap();
        let cap = re.find_iter(input).collect::<Vec<Match>>();
        let left: [String; 10] = (0..10)
            .into_iter()
            .map(|v| cap[v].as_str().to_owned())
            .collect::<Vec<String>>()
            .try_into()
            .unwrap_or_else(|v: Vec<String>| {
                panic!("Expected a Vec of length {} but it was {}", 10, v.len())
            });
        let right: [String; 4] = (10..14)
            .into_iter()
            .map(|v| cap[v].as_str().to_owned())
            .collect::<Vec<String>>()
            .try_into()
            .unwrap_or_else(|v: Vec<String>| {
                panic!("Expected a Vec of length {} but it was {}", 4, v.len())
            });

        Ok(SevenSegementDisplay { left, right })
    }
}

impl SevenSegementDisplay {
    fn determine_mapping(&self) -> HashMap<char, char> {
        let mut my_map = HashMap::new();

        let mut the_one: Vec<char> = [].to_vec();
        let mut the_seven: Vec<char> = [].to_vec();
        let mut the_four: Vec<char> = [].to_vec();
        let mut the_eight: Vec<char> = [].to_vec();
        for no in &self.left {
            match no.len() {
                2 => {
                    the_one = no.chars().collect();
                }
                3 => {
                    the_seven = no.chars().collect();
                }
                4 => {
                    the_four = no.chars().collect();
                }
                5 => {} //2,5,3}
                6 => {} //0,6,9}
                7 => {
                    the_eight = no.chars().collect();
                }
                _ => {}
            }
        }
        println!("{:?}", self.left);

        // Whatever character is in the '7' which is not in the '1' maps to 'a'
        let mut the_char_that_maps_to_a = 'a';
        for c in the_seven.iter() {
            if !the_one.contains(c) {
                the_char_that_maps_to_a = *c;
                my_map.insert(*c, 'a');
            }
        }

        // Only the 8 and 9 contain all the digits of the '4', so let's find the '9', and hence what maps to 'e'
        let mut the_nine: Vec<char> = [].to_vec();
        for poss in self.left.iter() {
            let poss_as_v: Vec<char> = poss.chars().collect();
            if poss_as_v.len() == 6 && the_four.iter().all(|v| poss_as_v.contains(v)) {
                the_nine = poss_as_v
            }
        }
        let mut the_char_that_maps_to_e = 'e';
        for c in the_eight.iter() {
            if !the_nine.contains(c) {
                the_char_that_maps_to_e = *c;
                my_map.insert(the_char_that_maps_to_e.to_owned(), 'e');
            }
        }

        // All three numbers that have five digits the '2', the '3' and the '5' use A, D, G in common
        // So we can find the two, becuase it's the only one that has the 'e'
        let five_digit_nos: Vec<Vec<char>> = self
            .left
            .iter()
            .filter(|v| v.len() == 5)
            .map(|v| v.chars().collect())
            .collect();
        let the_two = five_digit_nos
            .iter()
            .find(|v| v.contains(&the_char_that_maps_to_e))
            .unwrap()
            .to_owned();

        // the five is the one with 2 letters 3
        let the_five_and_six: Vec<Vec<char>> = five_digit_nos
            .iter()
            .filter(|v| !v.contains(&the_char_that_maps_to_e))
            .map(|v| v.to_owned())
            .collect();

        // This A, D, and G
        let common_digits_across_two_and_five_and_six: Vec<char> = the_two
            .iter()
            .filter(|v| the_five_and_six[0].contains(v))
            .filter(|v| the_five_and_six[1].contains(v))
            .copied()
            .collect_vec();

        // Using the '1' we can find which one maps to 'f' and 'c', by using the '2'
        let mut the_char_that_maps_to_c = 'c';
        let mut the_char_that_maps_to_f = 'f';

        for c in the_one.iter() {
            if the_two.contains(c) {
                the_char_that_maps_to_c = *c;
                my_map.insert(*c, 'c');
            } else {
                the_char_that_maps_to_f = *c;
                my_map.insert(*c, 'f');
            }
        }

        let the_five = five_digit_nos
            .iter()
            .find(|v| !v.contains(&the_char_that_maps_to_c))
            .unwrap()
            .to_owned();

        // Using the '4' we can find which one maps to 'd'
        let mut the_char_that_maps_to_d = 'h';
        for c in the_four.iter() {
            if common_digits_across_two_and_five_and_six.contains(c) {
                the_char_that_maps_to_d = *c;
                my_map.insert(*c, 'd');
            }
        }

        // Using thr '5' and '2' and the one which maps to 'd', we can find 'b' and 'g'
        for c in the_five.iter() {
            if c != &the_char_that_maps_to_d
                && c != &the_char_that_maps_to_a
                && c != &the_char_that_maps_to_f
            {
                if the_two.contains(c) {
                    my_map.insert(*c, 'g');
                } else {
                    my_map.insert(*c, 'b');
                }
            }
        }
        my_map
    }

    fn right_sum(&self) -> i32 {
        let the_mapping: HashMap<char, char> = self.determine_mapping();
        println!("{:?}", the_mapping);
        self.right
            .iter()
            .map(|v| {
                let mut mapped_chars: Vec<char> = v
                    .chars()
                    .map(|u| *the_mapping.get(&u).unwrap())
                    .collect::<Vec<char>>();
                mapped_chars.sort_unstable();
                let sorted_string: String = mapped_chars.iter().collect();
                let digit_it_represents = ATLAS.iter().position(|&x| x == sorted_string).unwrap();
                char::from_digit(digit_it_represents as u32, 10).unwrap()
            })
            .collect::<String>()
            .parse::<i32>()
            .unwrap()
    }
}

fn part_one(seven_segment_displays: &[SevenSegementDisplay]) {
    let count: usize = seven_segment_displays
        .iter()
        .map(|v| &v.right)
        .map(|v| v.iter().filter(|u| [2, 3, 4, 7].contains(&u.len())).count())
        .sum();
    println!("Part 1 answer is {}", count);
}

fn part_two(seven_segment_displays: &[SevenSegementDisplay]) {
    let count: i32 = seven_segment_displays.iter().map(|v| v.right_sum()).sum();
    println!("Part 2 answer is {}", count);
}

pub(crate) fn day08() {
    // Load inputs from input directory
    let seven_segment_displays: Vec<SevenSegementDisplay> = util::load_inputs("08".to_string())
        .iter()
        .map(|v| SevenSegementDisplay::from_str(v).unwrap())
        .collect();

    part_one(&seven_segment_displays);
    part_two(&seven_segment_displays);
}
