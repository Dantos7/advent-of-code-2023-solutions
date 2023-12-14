use counter::Counter;
use tqdm::tqdm;
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    let char_mat = build_matrix(input);
    let tilted_char_mat = tilt_north(&char_mat);
    let rows = tilted_char_mat.len();
    for (i, row) in tilted_char_mat.iter().enumerate() {
        let weight = rows - i;
        let count_round = row.iter().filter(|&&c| c == 'O').count();
        sum += weight * count_round;
    }

    Some(sum)
}

fn build_matrix(input: &str) -> Vec<Vec<char>> {
    let mut matrix = vec![];
    for line in input.split("\n") {
        if line.len() > 0 {
            let char_vec: Vec<char> = line.chars().collect();
            matrix.push(char_vec);
        }
    }
    matrix
}

fn tilt_north(char_mat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let reversed_char_mat = reverse_mat(char_mat);
    let mut tilted_reversed_char_mat = vec![];
    for row in reversed_char_mat.iter() {
        tilted_reversed_char_mat.push(shift_row_left(row));
    }
    reverse_mat(&tilted_reversed_char_mat)
}

fn reverse_mat(char_mat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let columns = char_mat[0].len();
    let rows = char_mat.len();
    let mut reversed_char_mat = vec![vec!['.'; rows]; columns];
    for i in 0..rows {
        for j in 0..columns {
            reversed_char_mat[j][i] = char_mat[i][j];
        }
    }
    reversed_char_mat
}

fn shift_row_left(row: &Vec<char>) -> Vec<char> {
    let mut new_row = row.clone();
    for (i, c) in row.iter().enumerate() {
        if *c == 'O' {
            shift_round_left(&mut new_row, i);
        }
    }
    new_row
}

/// PRE-CONDITION positions before i have been already shifted left
/// POST-CONDITION the round at position i has been shifted in-place to the leftmost position possible
fn shift_round_left(new_row: &mut Vec<char>, start: usize) {
    let mut i = start;
    while i > 0 {
        let left_char = new_row[i - 1];
        if left_char == '.' {
            new_row[i] = '.';
            new_row[i - 1] = 'O';
        } else {
            break;
        }
        i -= 1;
    }
}

/// IDEA: doing 1000000000 cycles is computationally unfeasible. We can assume that at some point the sums will stabilize in a recurrent cycle.
/// Hence, do 10000 cycles (~1 minute, assumption: this is enough for the sum to stabilize) and save the sums.
/// Use a Counter to detect sums occurring significantly more than others (part of the cycle)
/// Try out sums in the cycle on the advent of code page (use the middle value to take advantage of advent of code hints)
pub fn part_two(input: &str) -> Option<usize> {
    let mut sums = vec![];
    let char_mat = build_matrix(input);
    let mut tilted_char_mat = char_mat.clone();
    for _ in tqdm(0..10000) {
        for direction in ["north", "west", "south", "east"] {
            tilted_char_mat = match direction {
                "north" => tilt_north(&tilted_char_mat),
                "west" => tilt_west(&tilted_char_mat),
                "south" => tilt_south(&tilted_char_mat),
                "east" => tilt_east(&tilted_char_mat),
                _ => panic!("Invalid direction"),
            };
        }
        let mut sum = 0;
        let rows = tilted_char_mat.len();
        for (i, row) in tilted_char_mat.iter().enumerate() {
            let weight = rows - i;
            let count_round = row.iter().filter(|&&c| c == 'O').count();
            sum += weight * count_round;
        }
        sums.push(sum);
    }

    // Count the occurrences to identify the cycle
    let counter = sums.iter().map(|&x| x).collect::<Counter<usize>>().most_common_ordered();
    for (v, c) in counter {
        // Don't print sums occurring only once as they are for sure not in the cycle
        if c > 1 {
            println!("{v}: {c}");
        }
    }

    // leave here None because there is a set of possible solutions and not a single solution (refer to stdout)
    // Solving this problem with an exact solution would require
    // - identifying the start index of the cycle,
    // - repeat it until less than 1000000000,
    // - count the steps to reach 1000000000 and then take the appropriate value in the cycle
    None
}

fn tilt_south(char_mat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let reversed_char_mat = reverse_mat(char_mat);
    let mut tilted_reversed_char_mat = vec![];
    for row in reversed_char_mat.iter() {
        tilted_reversed_char_mat.push(shift_row_right(row));
    }
    reverse_mat(&tilted_reversed_char_mat)
}

fn tilt_east(char_mat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_char_mat = vec![];
    for row in char_mat.iter() {
        tilted_char_mat.push(shift_row_right(row));
    }
    tilted_char_mat
}

fn tilt_west(char_mat: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_char_mat = vec![];
    for row in char_mat.iter() {
        tilted_char_mat.push(shift_row_left(row));
    }
    tilted_char_mat
}

fn shift_row_right(row: &Vec<char>) -> Vec<char> {
    let mut new_row = row.clone();
    for (i, c) in row.iter().enumerate().rev() {
        if *c == 'O' {
            shift_round_right(&mut new_row, i);
        }
    }
    new_row
}

/// PRE-CONDITION positions after i have been already shifted right
/// POST-CONDITION the round at position i has been shifted in-place to the rightmost position possible
fn shift_round_right(new_row: &mut Vec<char>, start: usize) {
    let mut i = start;
    while i < (new_row.len() - 1) {
        let right_char = new_row[i + 1];
        if right_char == '.' {
            new_row[i] = '.';
            new_row[i + 1] = 'O';
        } else {
            break;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
