use std::cmp;

fn part_one() {
    let mut player_a_score = 0;
    let mut player_a_position = 6;
    let mut player_b_score = 0;
    let mut player_b_position = 3;
    let mut next_roll = 1;

    while player_a_score < 1000 && player_b_score < 1000 {
        player_a_position = (player_a_position + (3 * next_roll) + 3) % 10;
        next_roll += 3;
        player_a_score += {
            if player_a_position == 0 {
                10
            } else {
                player_a_position
            }
        };

        if player_a_score >= 1000 {
            break;
        }

        player_b_position = (player_b_position + (3 * next_roll) + 3) % 10;
        next_roll += 3;
        player_b_score += {
            if player_b_position == 0 {
                10
            } else {
                player_b_position
            }
        };
    }
    println!(
        "The answer to part one is {}",
        (next_roll - 1) * cmp::min(player_a_score, player_b_score)
    );
}

fn new_score(position: u8, _move: u8) -> u8 {
    if (position + _move) % 10 == 0 {
        10
    } else {
        (position + _move) % 10
    }
}

fn count_winners_from(
    player_a_score: u8,
    player_a_position: u8,
    player_b_score: u8,
    player_b_position: u8,
    next_turn: char,
) -> [usize; 2] {
    if player_a_score >= 21 {
        return [1, 0];
    } else if player_b_score >= 21 {
        return [0, 1];
    }
    let mut rc: [usize; 2] = [0, 0];
    if next_turn == 'a' {
        // 1 roll is +3/+9, 3 is + 4/+8, 6 is +5/+7, 10 is +6
        let [a_wins, b_wins] = count_winners_from(
            player_a_score + new_score(player_a_position, 3),
            (player_a_position + 3) % 10,
            player_b_score,
            player_b_position,
            'b',
        );
        rc[0] += a_wins;
        rc[1] += b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score + new_score(player_a_position, 4),
            (player_a_position + 4) % 10,
            player_b_score,
            player_b_position,
            'b',
        );
        rc[0] += 3 * a_wins;
        rc[1] += 3 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score + new_score(player_a_position, 5),
            (player_a_position + 5) % 10,
            player_b_score,
            player_b_position,
            'b',
        );
        rc[0] += 6 * a_wins;
        rc[1] += 6 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score + new_score(player_a_position, 6),
            (player_a_position + 6) % 10,
            player_b_score,
            player_b_position,
            'b',
        );
        rc[0] += 7 * a_wins;
        rc[1] += 7 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score + new_score(player_a_position, 7),
            (player_a_position + 7) % 10,
            player_b_score,
            player_b_position,
            'b',
        );
        rc[0] += 6 * a_wins;
        rc[1] += 6 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score + new_score(player_a_position, 8),
            (player_a_position + 8) % 10,
            player_b_score,
            player_b_position,
            'b',
        );
        rc[0] += 3 * a_wins;
        rc[1] += 3 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score + new_score(player_a_position, 9),
            (player_a_position + 9) % 10,
            player_b_score,
            player_b_position,
            'b',
        );
        rc[0] += a_wins;
        rc[1] += b_wins;
    } else {
        // 111 only (1)/112 (3)/122 or 113 (6)/??/
        // 1 roll is +3/+9, 3 is + 4/+8, 6 is +5/+7, 10 is +6

        let [a_wins, b_wins] = count_winners_from(
            player_a_score,
            player_a_position,
            player_b_score + new_score(player_b_position, 3),
            (player_b_position + 3) % 10,
            'a',
        );
        rc[0] += a_wins;
        rc[1] += b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score,
            player_a_position,
            player_b_score + new_score(player_b_position, 4),
            (player_b_position + 4) % 10,
            'a',
        );
        rc[0] += 3 * a_wins;
        rc[1] += 3 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score,
            player_a_position,
            player_b_score + new_score(player_b_position, 5),
            (player_b_position + 5) % 10,
            'a',
        );
        rc[0] += 6 * a_wins;
        rc[1] += 6 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score,
            player_a_position,
            player_b_score + new_score(player_b_position, 6),
            (player_b_position + 6) % 10,
            'a',
        );
        rc[0] += 7 * a_wins;
        rc[1] += 7 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score,
            player_a_position,
            player_b_score + new_score(player_b_position, 7),
            (player_b_position + 7) % 10,
            'a',
        );
        rc[0] += 6 * a_wins;
        rc[1] += 6 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score,
            player_a_position,
            player_b_score + new_score(player_b_position, 8),
            (player_b_position + 8) % 10,
            'a',
        );
        rc[0] += 3 * a_wins;
        rc[1] += 3 * b_wins;
        let [a_wins, b_wins] = count_winners_from(
            player_a_score,
            player_a_position,
            player_b_score + new_score(player_b_position, 9),
            (player_b_position + 9) % 10,
            'a',
        );
        rc[0] += a_wins;
        rc[1] += b_wins;
    }
    rc
}

fn part_two() {
    println!(
        "The answer to part two is {}",
        count_winners_from(0, 6, 0, 3, 'a')
            .to_vec()
            .iter()
            .max()
            .unwrap()
    );
}

pub(crate) fn day21() {
    part_one();
    part_two();
}
