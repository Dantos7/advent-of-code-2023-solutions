advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let char_mat = build_matrix(input);

    for (i, char_vec) in char_mat.iter().enumerate() {
        let mut digits_run = String::from("");
        let mut is_number_adjacent = false;
        for (j, c) in char_vec.iter().enumerate() {
            if c.is_digit(10) {
                digits_run.push(*c);
                if is_adjacent_to_symbol(i,j, &char_mat) {
                    is_number_adjacent = true;
                }
            } else {
                if is_number_adjacent {
                    // At least one digit is adjacent to a symbol (if run is empty, is_number_adjacent is false)
                    sum += digits_run.parse::<u32>().unwrap();
                }
                digits_run = String::from("");
                is_number_adjacent = false;
            }
        }
        // Must check also at the end of the line for numbers at the right border
        if is_number_adjacent {
            // At least one digit is adjacent to a symbol (if run is empty, is_number_adjacent is false)
            sum += digits_run.parse::<u32>().unwrap();
        }
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

fn is_adjacent_to_symbol(i: usize, j: usize, mat: &Vec<Vec<char>>) -> bool {
    let i_ = i as i32;
    let j_ = j as i32;
    let max_i = (mat.len() - 1) as i32;
    let max_j = (mat[0].len() - 1) as i32;
    let mut is_adjacent = false;
    let adjacent_cells = [(i_+1,j_), (i_+1,j_+1), (i_-1,j_), (i_-1,j_-1), (i_,j_+1), (i_,j_-1), (i_-1,j_+1), (i_+1,j_-1)];
    for (neighbor_i, neighbor_j) in adjacent_cells {
        if (0 <= neighbor_i && neighbor_i <= max_i) && (0 <= neighbor_j && neighbor_j <= max_j) {
            // the neighbor cell is a valid coordinate
            if is_symbol(mat[neighbor_i as usize][neighbor_j as usize]) {
                is_adjacent = true;
                break;
            };
        }
    }
    is_adjacent
}

fn is_symbol(c:char) -> bool {
    c != '.' && !c.is_digit(10)
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
