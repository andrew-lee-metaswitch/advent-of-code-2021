use crate::util;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
struct SyntaxString(String);

impl FromStr for SyntaxString {
    type Err = regex::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(SyntaxString(input.to_string()))
    }
}

impl SyntaxString {
    fn simplified_str(&self) -> String {
        let mut string_broken = self.0.clone();
        while string_broken.contains("{}")
            || string_broken.contains("()")
            || string_broken.contains("<>")
            || string_broken.contains("[]")
        {
            string_broken = string_broken.replace("{}", "");
            string_broken = string_broken.replace("()", "");
            string_broken = string_broken.replace("[]", "");
            string_broken = string_broken.replace("<>", "");
        }
        string_broken
    }

    fn is_corrupting(&self) -> bool {
        let string_broken = self.simplified_str();
        string_broken.contains('}')
            || string_broken.contains(')')
            || string_broken.contains('>')
            || string_broken.contains(']')
    }

    fn syntax_score(&self) -> i32 {
        let mut string_broken = self.simplified_str();
        string_broken = string_broken.replace("{", "");
        string_broken = string_broken.replace("(", "");
        string_broken = string_broken.replace("[", "");
        string_broken = string_broken.replace("<", "");
        let first_illegal_character = string_broken.chars().next().unwrap();
        match first_illegal_character {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        }
    }

    fn total_points(&self) -> i64 {
        // This will be of the form [{<((({ say
        let mut total: i64 = 0;
        for c in self.simplified_str().chars().rev() {
            total *= 5;
            total += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
        }
        total
    }
}

fn part_one(syntax_strs: Vec<SyntaxString>) {
    println!(
        "Part One: Syntax Score sum is {} ",
        syntax_strs
            .iter()
            .filter(|v| v.is_corrupting())
            .map(|v| v.syntax_score())
            .sum::<i32>()
    );
}

fn part_two(syntax_strs: Vec<SyntaxString>) {
    let mut total_scores: Vec<i64> = syntax_strs
        .iter()
        .filter(|v| !v.is_corrupting())
        .map(|v| v.total_points())
        .collect();
    total_scores.sort_unstable();
    let mid = total_scores.len() / 2;

    println!("Part Two: Median Total Score sum is {} ", total_scores[mid]);
}

pub(crate) fn day10() {
    // Load inputs from input directory
    let syntax_strs: Vec<SyntaxString> = util::load_inputs("10".to_string())
        .iter()
        .map(|v| SyntaxString::from_str(v).unwrap())
        .collect();

    part_one(syntax_strs.clone());
    part_two(syntax_strs);
}
