use crate::util;

fn orthogonal_points(point: &[usize; 2], lava_tubes: &[Vec<u8>]) -> Vec<[usize; 2]> {
    let [row, column] = point;
    let mut orthogonal_points: Vec<[usize; 2]> = [].to_vec();
    if *row != 0 {
        orthogonal_points.push([row - 1, *column])
    }
    if *row != lava_tubes.len() - 1 {
        orthogonal_points.push([row + 1, *column])
    }
    if *column != 0 {
        orthogonal_points.push([*row, column - 1])
    }
    if *column != lava_tubes[0].len() - 1 {
        orthogonal_points.push([*row, column + 1])
    }
    orthogonal_points
}

fn part_one(lava_tubes: &[Vec<u8>]) {
    let mut low_points: Vec<u8> = [].to_vec();
    for row in 0..lava_tubes.len() {
        for column in 0..lava_tubes[0].len() {
            let orthogonal_points = orthogonal_points(&[row, column], lava_tubes);
            //println!("ortho points are: {:?}", orthogonal_points);
            if orthogonal_points
                .iter()
                .all(|[u, v]| lava_tubes[*u][*v] > lava_tubes[row][column])
            {
                low_points.push(lava_tubes[row][column])
            }
        }
    }
    println!(
        "The part one answer is {}",
        low_points.len() as u32 + low_points.iter().map(|v| *v as u32).sum::<u32>()
    );
}

fn part_two(lava_tubes: &[Vec<u8>]) {
    let mut basin_sizes: Vec<usize> = [].to_vec();
    for row in 0..lava_tubes.len() {
        for column in 0..lava_tubes[0].len() {
            let my_orthogonal_points = orthogonal_points(&[row, column], lava_tubes);
            if my_orthogonal_points
                .iter()
                .all(|[u, v]| lava_tubes[*u][*v] > lava_tubes[row][column])
            {
                let mut basin_points: Vec<[usize; 2]> = [[row, column]].to_vec();
                let mut un_discovered_basin_points: Vec<[usize; 2]> = [[row, column]].to_vec();

                while let Some(p) = un_discovered_basin_points.pop() {
                    // p is a point in the basin
                    let orthogonal_points_to_p = orthogonal_points(&p, lava_tubes);
                    // For one of these orthogonal points, q, to be in the basin, it needs:
                    // (a) it must be not of height '9'
                    // (b) every point r orthogonal to r that is less than it must be in the basin
                    // We may end up with the following situation:
                    // 3 2
                    // 2 1
                    // That 1 is in the basin, we'll look at 2s add it to the basin, then
                    // Add the 3.

                    let new_basin_points: Vec<[usize; 2]> = orthogonal_points_to_p
                        .iter()
                        .filter(|q| lava_tubes[q[0]][q[1]] != 9)
                        .filter(|q| {
                            orthogonal_points(q, lava_tubes)
                                .iter()
                                .filter(|[u, v]| lava_tubes[*u][*v] < lava_tubes[q[0]][q[1]])
                                .all(|[u, v]| basin_points.contains(&[*u, *v]))
                        })
                        .filter(|q| !basin_points.contains(q))
                        .copied()
                        .collect();
                    for new_point in new_basin_points.iter() {
                        un_discovered_basin_points.push(*new_point);
                        basin_points.push(*new_point);
                    }
                }

                // We've now found the basin
                basin_points.dedup();
                basin_sizes.push(basin_points.len());
            }
        }
    }
    basin_sizes.sort_unstable();
    println!(
        "The part two answer is {}",
        basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()
    );
}

pub(crate) fn day09() {
    // Load inputs from input directory
    let lava_tubes: Vec<Vec<u8>> = util::load_inputs("09".to_string())
        .iter()
        .map(|x| x.chars().map(|v| v.to_digit(10).unwrap() as u8).collect())
        .collect();

    part_one(&lava_tubes);
    part_two(&lava_tubes)
}
