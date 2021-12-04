use itertools::all;

use crate::util;

#[derive(Clone, Debug, PartialEq, Eq)]
struct BingoBoard {
    values: Vec<i8>,
    called: Vec<bool>,
}

impl BingoBoard {
    fn from_vec(input: &[String]) -> Self {
        let mut values: Vec<i8> = [].to_vec();
        for input_line in input {
            for value in input_line.split_whitespace().into_iter() {
                values.push(value.parse().unwrap())
            }
        }

        BingoBoard {
            values,
            called: vec![false; 25],
        }
    }

    fn sum_of_unturned(&self) -> i32 {
        self.values
            .iter()
            .zip(self.called.iter())
            .filter(|v| !v.1)
            .map(|v| *v.0 as i32)
            .sum::<i32>()
    }

    fn has_won(&self) -> bool {
        let five_spaces: Vec<[i32; 5]> = [
            // Rows
            [0, 1, 2, 3, 4],
            [5, 6, 7, 8, 9],
            [10, 11, 12, 13, 14],
            [15, 16, 17, 18, 19],
            [20, 21, 22, 23, 24],
            // Columns
            [0, 5, 10, 15, 20],
            [1, 6, 11, 16, 21],
            [2, 7, 12, 17, 22],
            [3, 8, 13, 18, 23],
            [4, 9, 14, 19, 24],
            // Diagonals - not needed
            // [0, 6, 12, 18, 24],
            // [4, 8, 12, 16, 20],
        ]
        .to_vec();

        five_spaces
            .iter()
            .any(|v| all(v.iter(), |u| self.called[*u as usize]))
    }
}

fn part_one(mut bingo_boards: Vec<BingoBoard>, bingo_calls: Vec<i8>) {
    for call in bingo_calls {
        for (pos1, bingo_board) in bingo_boards.clone().iter().enumerate() {
            // mutable bingo board is at bingo_boards[pos1]

            for (pos, e) in bingo_board.values.iter().enumerate() {
                if e == &call {
                    bingo_boards[pos1].called[pos] = true
                }
            }

            if BingoBoard::has_won(&bingo_boards[pos1]) {
                //"We've won"
                println!("The game has finished");
                println!("The game board was {:?}", bingo_boards[pos1]);
                let sum_of_unturned = bingo_boards[pos1].sum_of_unturned();
                println!(
                    "The sum was {:?}, the number just called was {}",
                    sum_of_unturned, call
                );
                println!(
                    "The game score was {}",
                    score = call as i32 * sum_of_unturned
                );
                return;
            }
        }
    }
}

fn part_two(mut bingo_boards: Vec<BingoBoard>, bingo_calls: Vec<i8>) {
    for call in bingo_calls {
        println!(
            "BINGO CALL of {}, only {} bingo boards still haven't won",
            call,
            bingo_boards.len()
        );

        for (pos1, bingo_board) in bingo_boards.clone().iter().enumerate() {
            // mutable bingo board is at bingo_boards[pos1]

            for (pos, e) in bingo_board.values.iter().enumerate() {
                if e == &call {
                    // println!("{} {} {}", pos1, pos, e);
                    bingo_boards[pos1].called[pos] = true
                }
            }
            if bingo_boards.len() == 1 && bingo_boards[0].has_won() {
                println!(
                    "The game score was {}",
                    score = call as i32 * bingo_boards[0].sum_of_unturned()
                );
                return;
            }
        }
        bingo_boards = bingo_boards.into_iter().filter(|v| !v.has_won()).collect()
    }
}

pub(crate) fn day04() {
    // Load inputs from input directory
    let mut bingo_boards = [].to_vec();
    let mut bingo_calls = [].to_vec();
    let mut current_bingo_board: Vec<String> = [].to_vec();
    for line in util::load_inputs("04".to_string()) {
        if line.contains(',') {
            bingo_calls = line.split(',').map(|v| v.parse::<i8>().unwrap()).collect();
        } else if line.is_empty() {
            if current_bingo_board.len() == 5 {
                bingo_boards.push(BingoBoard::from_vec(&current_bingo_board));
                current_bingo_board = [].to_vec();
            }
        } else {
            current_bingo_board.push(line)
        }
    }
    bingo_boards.push(BingoBoard::from_vec(&current_bingo_board));

    part_one(bingo_boards.clone(), bingo_calls.clone());
    part_two(bingo_boards.clone(), bingo_calls);
}
