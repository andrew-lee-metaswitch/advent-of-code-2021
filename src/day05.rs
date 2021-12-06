use crate::util;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Vent {
    points: Vec<[i32; 2]>,
    is_orthogonal: bool,
}

impl FromStr for Vent {
    type Err = regex::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"(\d*),(\d*) -> (\d*),(\d*)"#).unwrap();

        if let Some(cap) = re.captures_iter(input).next() {
            // Captures({0: Some("781,721 -> 781,611"), 1: Some("781"), 2: Some("721"), 3: Some("781"), 4: Some("611")})
            let start_point: [i32; 2] = [
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            ];
            let end_point: [i32; 2] = [
                cap[3].parse::<i32>().unwrap(),
                cap[4].parse::<i32>().unwrap(),
            ];
            if start_point[0] == end_point[0] {
                if start_point[1] > end_point[1] {
                    return Ok(Vent {
                        points: (end_point[1]..start_point[1] + 1)
                            .map(|v| [start_point[0], v])
                            .collect(),
                        is_orthogonal: true,
                    });
                } else {
                    return Ok(Vent {
                        points: (start_point[1]..end_point[1] + 1)
                            .map(|v| [start_point[0], v])
                            .collect(),
                        is_orthogonal: true,
                    });
                }
            } else if start_point[1] == end_point[1] {
                if start_point[0] > end_point[0] {
                    return Ok(Vent {
                        points: (end_point[0]..start_point[0] + 1)
                            .map(|v| [v, start_point[1]])
                            .collect(),
                        is_orthogonal: true,
                    });
                } else {
                    return Ok(Vent {
                        points: (start_point[0]..end_point[0] + 1)
                            .map(|v| [v, start_point[1]])
                            .collect(),
                        is_orthogonal: true,
                    });
                }
            } else {
                let x_delta = end_point[0] - start_point[0];
                let gradient = (end_point[1] - start_point[1]) as f32 / x_delta as f32;
                let mut points: Vec<[i32; 2]> = [start_point].to_vec();
                if x_delta < 0 {
                    for x in 1..(-x_delta + 1) {
                        let y = x as f32 * gradient;

                        if y.fract() == 0.0 {
                            let y_as_i32 = y.round() as i32;
                            points.push([start_point[0] - x, start_point[1] - y_as_i32])
                        }
                    }
                } else {
                    for x in 1..x_delta + 1 {
                        let y = x as f32 * gradient;

                        if y.fract() == 0.0 {
                            let y_as_i32 = y.round() as i32;
                            points.push([start_point[0] + x, start_point[1] + y_as_i32])
                        }
                    }
                }
                return Ok(Vent {
                    points,
                    is_orthogonal: false,
                });
            }
        }
        Err(regex::Error::Syntax("Bad things happened".to_string()))
    }
}

fn part_one(vents: Vec<Vent>) {
    let mut points_hash: HashMap<[i32; 2], i32> = HashMap::new();
    for point in vents
        .into_iter()
        .filter(|v| v.is_orthogonal)
        .map(|v| v.points)
        .flatten()
    {
        match points_hash.clone().get(&point) {
            Some(v) => points_hash.insert(point, *v + 1),
            None => points_hash.insert(point, 1),
        };
    }

    let points_on_multiple_vents = points_hash.values().filter(|v| *v > &1).count();
    println!("Answer to part 1 is {} vents", points_on_multiple_vents);
}

fn part_two(vents: Vec<Vent>) {
    let mut points_hash: HashMap<[i32; 2], i32> = HashMap::new();
    for point in vents.into_iter().map(|v| v.points).flatten() {
        match points_hash.clone().get(&point) {
            Some(v) => points_hash.insert(point, *v + 1),
            None => points_hash.insert(point, 1),
        };
    }

    let points_on_multiple_vents = points_hash.values().filter(|v| *v > &1).count();
    println!("Answer to part 2 is {} vents", points_on_multiple_vents);
}

pub(crate) fn day05() {
    // Load inputs from input directory
    let vents: Vec<Vent> = util::load_inputs("05".to_string())
        .iter()
        .map(|v| Vent::from_str(v).unwrap())
        .collect();

    part_one(vents.clone());
    part_two(vents);
}
