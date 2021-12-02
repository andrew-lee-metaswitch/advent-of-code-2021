use crate::util;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Direction {
    Forward,
    Up,
    Down,
}

struct Instruction {
    direction: Direction,
    amount: i32,
}

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (d, a) = input.split_once(" ").unwrap();

        let direction = match d {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Whoops-si-daisy!"),
        };
        let amount = a.parse::<i32>().unwrap();
        Ok(Instruction { direction, amount })
    }
}

fn part_one(submarine_instructions: &[Instruction]) {
    let depth: i32 = submarine_instructions
        .iter()
        .filter(|v| v.direction == Direction::Down)
        .map(|v| v.amount)
        .sum::<i32>()
        - submarine_instructions
            .iter()
            .filter(|v| v.direction == Direction::Up)
            .map(|v| v.amount)
            .sum::<i32>();
    let horizonital_distance: i32 = submarine_instructions
        .iter()
        .filter(|v| v.direction == Direction::Forward)
        .map(|v| v.amount)
        .sum();

    println!(
        "The end position will be {} depth, {} forward, hence answer is {}",
        depth,
        horizonital_distance,
        depth * horizonital_distance
    );
}

fn part_two(submarine_instructions: &[Instruction]) {
    let mut aim = 0;
    let mut depth: i32 = 0;
    let mut horizonital_distance: i32 = 0;
    for instruction in submarine_instructions {
        match instruction.direction {
            Direction::Down => {
                aim += instruction.amount;
            }
            Direction::Up => {
                aim -= instruction.amount;
            }
            Direction::Forward => {
                horizonital_distance += instruction.amount;
                depth += instruction.amount * aim;
            }
        };
    }
    println!(
        "The end position will be {} depth, {} forward, hence answer is {}",
        depth,
        horizonital_distance,
        depth * horizonital_distance
    );
}

pub(crate) fn day02() {
    // Load inputs from input directory
    let submarine_instructions: Vec<Instruction> = util::load_inputs("02".to_string())
        .iter()
        .map(|v| Instruction::from_str(v).unwrap())
        .collect();

    part_one(&submarine_instructions);
    part_two(&submarine_instructions);
}
