advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    let char_mat = build_matrix(input);
    let reversed_char_mat = reverse_mat(char_mat);
    let mut tilted_reversed_char_mat = vec![];
    for row in reversed_char_mat.iter() {
        tilted_reversed_char_mat.push(shift_row_left(row));
    }
    let tilted_char_mat = reverse_mat(tilted_reversed_char_mat);
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

fn reverse_mat(char_mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
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

pub fn part_two(input: &str) -> Option<usize> {
    None
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
