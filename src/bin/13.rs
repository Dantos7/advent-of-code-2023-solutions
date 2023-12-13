advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for pattern_str in input.split("\n\n") {
        let pattern_char_mat = build_matrix(pattern_str);
        let j_split = get_vertical_symmetry_index(&pattern_char_mat);
        let i_split = get_horizontal_symmetry_index(&pattern_char_mat);
        let columns_left = match j_split {
            Some(j) => j,
            None => 0,
        };
        let rows_above = match i_split {
            Some(i) => i,
            None => 0,
        };
        sum += rows_above * 100 + columns_left;
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

fn get_vertical_symmetry_index(pattern_char_mat: &Vec<Vec<char>>) -> Option<usize> {
    // index at 1 means that column 0 is equal to column 1
    // index at 2 means that column 1 is equal to column 2 and column 0 is equal to column 3
    // ...
    let mut j = 1;
    let columns_count = pattern_char_mat[0].len();
    let mut all_rows_symmetry;
    while j < columns_count {
        // check vertical symmetry in each line
        all_rows_symmetry = true;
        for row in pattern_char_mat.iter() {
            // check vertical symmetry in this row
            let mut offset = 0;
            let mut left_index = j - 1 - offset;
            let mut right_index = j + offset;
            let mut row_symmetry = row[left_index] == row[right_index];
            while row_symmetry && left_index > 0 && right_index < (columns_count - 1) {
                offset += 1;
                left_index = j - 1 - offset;
                right_index = j + offset;
                row_symmetry = row_symmetry && (row[left_index] == row[right_index]);
            }
            all_rows_symmetry = all_rows_symmetry && row_symmetry;
            if !all_rows_symmetry {
                break;
            }
        }
        if all_rows_symmetry {
            return Some(j);
        } else {
            j += 1;
        }
    }
    None
}

fn get_horizontal_symmetry_index(pattern_char_mat: &Vec<Vec<char>>) -> Option<usize> {
    // index at 1 means that row 0 is equal to row 1
    // index at 2 means that row 1 is equal to row 2 and row 0 is equal to row 3
    // ...
    let mut i = 1;
    let rows_count = pattern_char_mat.len();
    while i < rows_count {
        let mut offset = 0;
        let mut above_index = i - 1 - offset;
        let mut below_index = i + offset;
        let mut symmetry = same_vector_values(&pattern_char_mat[above_index], &pattern_char_mat[below_index]);
        while symmetry && above_index > 0 && below_index < (rows_count - 1) {
            offset += 1;
            above_index = i - 1 - offset;
            below_index = i + offset;
            symmetry = same_vector_values(&pattern_char_mat[above_index], &pattern_char_mat[below_index]);
        }
        if symmetry {
            return Some(i);
        } else {
            i += 1;
        }
    }
    None
}

fn same_vector_values(vec_1: &Vec<char>, vec_2: &Vec<char>) -> bool {
    if vec_1.len() != vec_2.len() {
        return false;
    }
    for i in 0..vec_1.len() {
        if vec_1[i] != vec_2[i] {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
