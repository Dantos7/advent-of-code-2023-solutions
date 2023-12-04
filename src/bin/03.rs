use std::collections::{HashMap, HashSet};

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
                if is_adjacent_to_symbol(i, j, &char_mat) {
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
    let adjacent_cells = [
        (i_ + 1, j_),
        (i_ + 1, j_ + 1),
        (i_ - 1, j_),
        (i_ - 1, j_ - 1),
        (i_, j_ + 1),
        (i_, j_ - 1),
        (i_ - 1, j_ + 1),
        (i_ + 1, j_ - 1),
    ];
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

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut parts_gears = HashMap::new();
    let char_mat = build_matrix(input);

    for (i, char_vec) in char_mat.iter().enumerate() {
        let mut digits_run = String::from("");
        let mut is_number_adjacent = false;
        let mut number_gears_coordinates: Vec<(usize, usize)> = vec![];
        for (j, c) in char_vec.iter().enumerate() {
            if c.is_digit(10) {
                digits_run.push(*c);
                let (is_adj, mut gears_coordinates) = is_adjacent_to_gear(i, j, &char_mat);
                if is_adj {
                    is_number_adjacent = true;
                    number_gears_coordinates.append(&mut gears_coordinates);
                }
            } else {
                if is_number_adjacent {
                    // At least one digit is adjacent to a symbol (if run is empty, is_number_adjacent is false)
                    parts_gears.insert((digits_run.parse::<u32>().unwrap(), i, j - 1), number_gears_coordinates.clone());
                }
                digits_run = String::from("");
                is_number_adjacent = false;
                number_gears_coordinates = vec![];
            }
        }
        // Must check also at the end of the line for numbers at the right border
        // (Double counting is avoided by resetting digits_run and is_number_adjacent after a successful detection)
        if is_number_adjacent {
            // At least one digit is adjacent to a symbol (if run is empty, is_number_adjacent is false)
            parts_gears.insert((digits_run.parse::<u32>().unwrap(), i, 140), number_gears_coordinates.clone());
        }
    }

    // Rearrange parts_gears in reverse (from gears to parts) in order to be able to compute the ratios
    let mut gears_parts: HashMap<(usize, usize), Vec<(u32, usize, usize)>> = HashMap::new();
    for (part, gears_cords) in parts_gears.iter() {
        for gear_cord in gears_cords {
            let parts = gears_parts.get(gear_cord);
            if parts.is_none() {
                gears_parts.insert(*gear_cord, vec![*part]);
            } else {
                let mut new_parts = parts?.clone();
                new_parts.push(*part);
                gears_parts.insert(*gear_cord, new_parts);
            }
        }
    }

    // Compute gear ratio sum
    for (_, parts) in gears_parts.iter_mut() {
        // Remove duplicates
        let set: HashSet<_> = parts.drain(..).collect(); // dedup
        parts.extend(set.into_iter());

        if parts.len() > 1 {
            let mut prod = 1;
            for p in parts.iter() {
                prod *= p.0;
            }
            sum += prod;
        }
    }
    Some(sum)
}

fn is_adjacent_to_gear(i: usize, j: usize, mat: &Vec<Vec<char>>) -> (bool, Vec<(usize, usize)>) {
    let i_ = i as i32;
    let j_ = j as i32;
    let max_i = (mat.len() - 1) as i32;
    let max_j = (mat[0].len() - 1) as i32;
    let mut is_adjacent = false;
    let mut gears_coordinates = vec![];
    let adjacent_cells = [
        (i_ + 1, j_),
        (i_ + 1, j_ + 1),
        (i_ - 1, j_),
        (i_ - 1, j_ - 1),
        (i_, j_ + 1),
        (i_, j_ - 1),
        (i_ - 1, j_ + 1),
        (i_ + 1, j_ - 1),
    ];
    for (neighbor_i, neighbor_j) in adjacent_cells {
        if (0 <= neighbor_i && neighbor_i <= max_i) && (0 <= neighbor_j && neighbor_j <= max_j) {
            // the neighbor cell is a valid coordinate
            if is_gear(mat[neighbor_i as usize][neighbor_j as usize]) {
                is_adjacent = true;
                gears_coordinates.push((neighbor_i as usize, neighbor_j as usize));
            };
        }
    }
    (is_adjacent, gears_coordinates)
}

fn is_gear(c: char) -> bool {
    c == '*'
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
        assert_eq!(result, Some(467835));
    }
}
