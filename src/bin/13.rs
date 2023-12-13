advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for pattern_str in input.split("\n\n") {
        let pattern_char_mat = build_matrix(pattern_str);
        let (j_split, _) = get_vertical_symmetry_index(&pattern_char_mat, false);
        let (i_split, _) = get_horizontal_symmetry_index(&pattern_char_mat, false);
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

fn get_vertical_symmetry_index(pattern_char_mat: &Vec<Vec<char>>, smudge: bool) -> (Option<usize>, Option<(usize, usize)>) {
    // index at 1 means that column 0 is equal to column 1
    // index at 2 means that column 1 is equal to column 2 and column 0 is equal to column 3
    // ...
    let mut j = 1;
    let columns_count = pattern_char_mat[0].len();
    let mut all_rows_symmetry;
    let mut smudge_coord = None;
    while j < columns_count {
        let mut already_found_smudge = false;
        // check vertical symmetry in each line
        all_rows_symmetry = true;
        for (i, row) in pattern_char_mat.iter().enumerate() {
            // check vertical symmetry in this row
            let mut offset = 0;
            let mut left_index = j - 1 - offset;
            let mut right_index = j + offset;
            let mut row_symmetry = row[left_index] == row[right_index];
            if !row_symmetry && smudge && !already_found_smudge {
                // check_vertical_symmetry_with_smudge
                if flip_symbol(row[left_index]) == row[right_index] {
                    smudge_coord = Some((i, left_index));
                    row_symmetry = true;
                    already_found_smudge = true;
                }
            }
            while row_symmetry && left_index > 0 && right_index < (columns_count - 1) {
                offset += 1;
                left_index = j - 1 - offset;
                right_index = j + offset;
                row_symmetry = row_symmetry && (row[left_index] == row[right_index]);
                if !row_symmetry && smudge && !already_found_smudge {
                    // check_vertical_symmetry_with_smudge
                    if flip_symbol(row[left_index]) == row[right_index] {
                        smudge_coord = Some((i, left_index));
                        row_symmetry = true;
                        already_found_smudge = true;
                    }
                }
            }
            all_rows_symmetry = all_rows_symmetry && row_symmetry;
            if !all_rows_symmetry {
                break;
            }
        }
        if all_rows_symmetry && (already_found_smudge || !smudge) {
            return (Some(j), smudge_coord);
        } else {
            j += 1;
        }
    }
    (None, None)
}

fn get_horizontal_symmetry_index(pattern_char_mat: &Vec<Vec<char>>, smudge: bool) -> (Option<usize>, Option<(usize, usize)>) {
    // index at 1 means that row 0 is equal to row 1
    // index at 2 means that row 1 is equal to row 2 and row 0 is equal to row 3
    // ...
    let mut i = 1;
    let rows_count = pattern_char_mat.len();
    let mut smudge_coord = None;
    while i < rows_count {
        let mut already_found_smudge = false;
        let mut offset = 0;
        let mut above_index = i - 1 - offset;
        let mut below_index = i + offset;
        let mut symmetry;
        let mut smudge_coord_j: Option<usize>;
        symmetry = same_vector_values(&pattern_char_mat[above_index], &pattern_char_mat[below_index]);
        if !symmetry && smudge && !already_found_smudge {
            (symmetry, smudge_coord_j) = same_vector_values_with_smudge(&pattern_char_mat[above_index], &pattern_char_mat[below_index]);
            if smudge_coord_j.is_some() {
                let j = smudge_coord_j.unwrap();
                smudge_coord = Some((above_index, j));
                already_found_smudge = true;
            }
        }
        while symmetry && above_index > 0 && below_index < (rows_count - 1) {
            offset += 1;
            above_index = i - 1 - offset;
            below_index = i + offset;
            let above_row = &pattern_char_mat[above_index];
            let below_row = &pattern_char_mat[below_index];
            symmetry = same_vector_values(above_row, below_row);
            if !symmetry && smudge && !already_found_smudge {
                (symmetry, smudge_coord_j) = same_vector_values_with_smudge(above_row, below_row);
                if symmetry {
                    let j = smudge_coord_j.unwrap();
                    smudge_coord = Some((above_index, j));
                    already_found_smudge = true;
                }
            }
        }
        if symmetry && (already_found_smudge || !smudge) {
            return (Some(i), smudge_coord);
        } else {
            i += 1;
        }
    }
    (None, smudge_coord)
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

fn same_vector_values_with_smudge(vec_1: &Vec<char>, vec_2: &Vec<char>) -> (bool, Option<usize>) {
    let mut already_found_smudge = false;
    let mut smudge_coord_j = None;
    if vec_1.len() != vec_2.len() {
        return (false, smudge_coord_j);
    }
    for j in 0..vec_1.len() {
        if vec_1[j] != vec_2[j] {
            if !already_found_smudge {
                if flip_symbol(vec_1[j]) != vec_2[j] {
                    return (false, smudge_coord_j);
                } else {
                    already_found_smudge = true;
                    smudge_coord_j = Some(j);
                }
            } else {
                return (false, smudge_coord_j);
            }
        }
    }
    (true, smudge_coord_j)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut sum = 0;
    for pattern_str in input.split("\n\n") {
        let pattern_char_mat = build_matrix(pattern_str);
        let j_split: Option<usize>;
        let j_split_smudged: Option<usize>;
        let j_split_valid: Option<usize>;
        let i_split: Option<usize>;
        let i_split_smudged: Option<usize>;
        let i_split_valid: Option<usize>;
        (j_split_smudged, _) = get_vertical_symmetry_index(&pattern_char_mat, true);
        (j_split, _) = get_vertical_symmetry_index(&pattern_char_mat, false);
        if j_split != j_split_smudged {
            j_split_valid = j_split_smudged;
        } else {
            j_split_valid = None;
        }

        (i_split_smudged, _) = get_horizontal_symmetry_index(&pattern_char_mat, true);
        (i_split, _) = get_horizontal_symmetry_index(&pattern_char_mat, false);
        if i_split != i_split_smudged {
            i_split_valid = i_split_smudged;
        } else {
            i_split_valid = None;
        }
        let columns_left = match j_split_valid {
            Some(j) => j,
            None => 0,
        };
        let rows_above = match i_split_valid {
            Some(i) => i,
            None => 0,
        };
        sum += rows_above * 100 + columns_left;
    }
    Some(sum)
}

fn flip_symbol(symbol: char) -> char {
    match symbol {
        '.' => '#',
        '#' => '.',
        _ => panic!("Invalid symbol"),
    }
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
        assert_eq!(result, Some(400));
    }
}
