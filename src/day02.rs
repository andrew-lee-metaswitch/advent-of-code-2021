use crate::util;

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

impl Instruction {
    fn from_str(input: &str) -> Instruction {
        let input_as_vec: Vec<&str> = input.split(' ').collect();

        let direction = match input_as_vec[0] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Whoops-si-daisy!"),
        };
        let amount = input_as_vec[1].parse::<i32>().unwrap();
        Instruction { direction, amount }
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
        .map(|v| Instruction::from_str(v))
        .collect();

    part_one(&submarine_instructions);
    part_two(&submarine_instructions);
}
