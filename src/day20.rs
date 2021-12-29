use crate::util;
use std::collections::HashMap;

fn enhance_image(
    image: HashMap<[i32; 2], bool>,
    translation: &[bool],
    new_cells_off: bool,
) -> HashMap<[i32; 2], bool> {
    let mut old_hm_with_new_cells = image.clone();
    for key in image.keys() {
        for y_delta in -1..2 {
            for x_delta in -1..2 {
                if image.get(&[key[0] + y_delta, key[1] + x_delta]) == None {
                    old_hm_with_new_cells
                        .insert([key[0] + y_delta, key[1] + x_delta], new_cells_off);
                }
            }
        }
    }
    let mut new_hm = HashMap::new();
    for key in old_hm_with_new_cells.keys() {
        let mut my_str: String = "".to_string();

        for y_delta in -1..2 {
            for x_delta in -1..2 {
                let new_char = match image.get(&[key[0] + y_delta, key[1] + x_delta]) {
                    Some(b) => {
                        if *b {
                            '1'
                        } else {
                            '0'
                        }
                    }
                    None => {
                        if new_cells_off {
                            '1'
                        } else {
                            '0'
                        }
                    }
                };
                my_str.push(new_char);
            }
        }
        let return_value = u16::from_str_radix(&my_str, 2).unwrap() as usize;
        let new_value = translation[return_value];
        new_hm.insert(*key, new_value);
    }
    new_hm
}

fn part_one(image: &HashMap<[i32; 2], bool>, translation: &[bool]) {
    println!(
        "The answer to part one is {}",
        image.values().filter(|v| **v).count()
    );
    let new_image = enhance_image(image.clone(), translation, false);
    let newer_image = enhance_image(new_image, translation, true);
    println!(
        "The answer to part one is {}",
        newer_image.values().filter(|v| **v).count()
    );
}

fn part_two(mut image: HashMap<[i32; 2], bool>, translation: &[bool]) {
    println!(
        "The answer to part one is {}",
        image.values().filter(|v| **v).count()
    );
    for i in 0..50 {
        let new_cells_off = i % 2 != 0;
        image = enhance_image(image, translation, new_cells_off);
    }
    println!(
        "The answer to part two is {}",
        image.values().filter(|v| **v).count()
    );
}

pub(crate) fn day20() {
    // Load inputs from input directory
    let (translation_vec, image) = match util::load_inputs("20".to_string()).split_first() {
        Some((first_line, rest)) => {
            let translation_vec: Vec<bool> = first_line.chars().map(|v| matches!(v, '#')).collect();
            let mut image: HashMap<[i32; 2], bool> = HashMap::new();
            for (row_id, row) in rest.iter().enumerate() {
                for (column_id, value) in row.chars().enumerate() {
                    image.insert([row_id as i32, column_id as i32], matches!(value, '#'));
                }
            }
            (translation_vec, image)
        }
        None => panic!("Bad things happeend"),
    };

    part_one(&image, &translation_vec);
    part_two(image, &translation_vec);
}
