use std::env;
use std::fs;

pub(crate) fn load_inputs(day_id: i32) -> Vec<String> {
    // This will path to advent-of-code-2020 directory

    let dir = env::current_dir().unwrap();
    let input_dir = dir.join("inputs");
    let input_file_name: String = format!("day{}.txt", day_id.to_string());
    let input_file = input_dir.join(input_file_name);

    let file_contents_str: String =
        fs::read_to_string(input_file).expect("Something went wrong reading the file");

    // Copied from RJW2
    file_contents_str
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
    // I had this before:
    // file_contents_str.split("\n").into_iter()..collect::<Vec<_>>()
}
