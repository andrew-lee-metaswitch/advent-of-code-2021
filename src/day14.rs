use itertools::Itertools;

use crate::util;
use std::collections::HashMap;

fn part_one(template: &str, conversions: &HashMap<String, char>) {
    let mut new_template = <&str>::clone(&template).to_owned();

    for _range in 0..10 {
        let mut new_formed_string = "".to_string();
        for position in 0..new_template.len() - 1 {
            // Push this character to the new_formed_string:
            new_formed_string.push(new_template.chars().nth(position).unwrap());

            // CHeck ofr match
            if let Some(new_char) =
                conversions.get(&new_template[position..position + 2].to_string())
            {
                new_formed_string.push(*new_char);
            }
        }
        // FInally push the last digit
        new_formed_string.push(new_template.chars().last().unwrap());
        new_template = new_formed_string;
    }

    let max = new_template
        .chars()
        .unique()
        .map(|c| new_template.chars().filter(|d| &c == d).count())
        .max()
        .unwrap();
    let min = new_template
        .chars()
        .unique()
        .map(|c| new_template.chars().filter(|d| &c == d).count())
        .min()
        .unwrap();
    println!("The answer to part one is {}", max - min);
}

fn part_two(template: &str, conversions: &HashMap<String, char>) {
    let mut count_of_pairs: HashMap<String, u64> = HashMap::new();
    for position in 0..template.len() - 1 {
        count_of_pairs
            .entry(template[position..position + 2].to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    for _range in 0..40 {
        //println!("COunt of pairs: {:?}", count_of_pairs);
        let mut new_count_of_pairs = count_of_pairs.clone();

        for (tuple, count) in count_of_pairs {
            // if tuple = "AB", new_char = 'c'

            if let Some(new_char) = conversions.get(&tuple) {
                // First we want Strings "AC" and "CB"
                let lhs = [tuple.chars().next().unwrap(), *new_char].iter().collect();
                let rhs = [*new_char, tuple.chars().nth(1).unwrap()].iter().collect();

                // Next we find add count to count_of_pairs[lhs] and count_of_pairs[rhs] respecitvely,
                new_count_of_pairs
                    .entry(lhs)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
                new_count_of_pairs
                    .entry(rhs)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
                // Next we remove 'count' number of the original tuple.
                new_count_of_pairs.entry(tuple).and_modify(|v| *v -= count);
            }
        }
        count_of_pairs = new_count_of_pairs;
    }

    let chars: Vec<char> = count_of_pairs
        .keys()
        .map(|c| c.chars().next().unwrap())
        .chain(count_of_pairs.keys().map(|c| c.chars().nth(1).unwrap()))
        .unique()
        .collect();

    let char_count: Vec<u64> = chars
        .iter()
        .map(|c| (count_of_pairs
                .iter()
                .filter(|(k, _v)| &k.chars().next().unwrap() == c)
                .map(|(_k, v)| v)
                .sum::<u64>()
                + count_of_pairs
                    .iter()
                    .filter(|(k, _v)| &k.chars().nth(1).unwrap() == c)
                    .map(|(_k, v)| v)
                    .sum::<u64>()           // we've double counted everything bar the first + last digit
             + { if template.chars().next().unwrap() == *c {1} else {0} } +  { if template.chars().last().unwrap() == *c {1} else {0}} ) /2

        )
        .collect();

    let max = char_count.iter().max().unwrap();
    let min = char_count.iter().min().unwrap();

    println!("The answer to part two is {}", max - min);
}

pub(crate) fn day14() {
    // Load inputs from input directory
    let mut conversions: HashMap<String, char> = HashMap::new();

    let lines = util::load_inputs("14".to_string());
    let template = lines.get(0).unwrap();

    for line in lines.iter() {
        if line.contains(" -> ") {
            let mut split_string = line.split(" -> ");
            conversions.insert(
                split_string.next().unwrap().to_string(),
                split_string
                    .next()
                    .unwrap()
                    .to_string()
                    .chars()
                    .next()
                    .unwrap(),
            );
        }
    }

    part_one(template, &conversions);
    part_two(template, &conversions);
}
