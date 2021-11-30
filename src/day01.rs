use crate::util;

static TARGET_SUM: i32 = 2020;

fn find_two_that_sum(expense_entries_vector: &Vec<String>) {
    // Loop over list
    for (s_position, s) in expense_entries_vector.iter().enumerate() {
        for (t_position, t) in expense_entries_vector.iter().enumerate() {
            if t_position > s_position {
                break;
            }
            let s_as_int: i32 = s.parse::<i32>().unwrap();
            let t_as_int: i32 = t.parse::<i32>().unwrap();

            if s_as_int + t_as_int == TARGET_SUM {
                println!(
                    "Whoop dee do {}, {} sum to {}",
                    s,
                    t,
                    TARGET_SUM.to_string()
                );
                println!("These multiply to {}", s_as_int * t_as_int);
                break;
            };
        }
    }
}

fn find_three_that_sum(expense_entries_vector: &Vec<String>) {
    for (s_position, s) in expense_entries_vector.iter().enumerate() {
        for (t_position, t) in expense_entries_vector.iter().enumerate() {
            for (u_position, u) in expense_entries_vector.iter().enumerate() {
                //println!("{} {}", s, t);
                if u_position > t_position {
                    break;
                }
                if t_position > s_position {
                    break;
                }

                let s_as_int: i32 = s.parse::<i32>().unwrap();
                let t_as_int: i32 = t.parse::<i32>().unwrap();
                let u_as_int: i32 = u.parse::<i32>().unwrap();

                if s_as_int + t_as_int + u_as_int == TARGET_SUM {
                    println!(
                        "Whoop dee do {}, {}, {} sum to {}",
                        s,
                        t,
                        u,
                        TARGET_SUM.to_string()
                    );
                    println!("These multiply to {}", s_as_int * t_as_int * u_as_int);
                    break;
                };
            }
        }
    }
}

pub(crate) fn day01() {
    // Load inputs from input directory
    let expense_entries_vector = util::load_inputs(01);

    find_two_that_sum(&expense_entries_vector);
    find_three_that_sum(&expense_entries_vector)
}
