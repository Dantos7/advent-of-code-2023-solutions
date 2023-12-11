use std::collections::HashMap;
advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    let mut char_mat = build_matrix(input);
    char_mat = expand_horizontally(char_mat);
    char_mat = expand_vertically(char_mat);
    let galaxies_coords = get_galaxies_coords(&char_mat);
    let distances = get_galaxies_distances(galaxies_coords);
    for (pair, d) in distances {
        sum += d;
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

fn expand_horizontally(char_mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_char_mat = char_mat.clone();
    let mut offset = 0; // use an offset to take into account insertions
    for (i, row) in char_mat.iter().enumerate() {
        if row.iter().filter(|c| **c == '#').collect::<Vec<&char>>().len() == 0 {
            new_char_mat.insert(offset + i, row.clone());
            offset += 1;
        }
    }
    new_char_mat
}

fn expand_vertically(char_mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut empty_columns = vec![true; char_mat[0].len()];
    let mut new_char_mat = char_mat.clone();
    for row in char_mat.iter() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                empty_columns[j] = false;
            }
        }
    }

    for (i, row) in char_mat.iter().enumerate() {
        let mut new_row = row.clone();
        let mut offset = 0; // use an offset to take into account insertions in the vector
        for (j, _) in row.iter().enumerate() {
            if empty_columns[j] {
                new_row.insert(offset + j, '.');
                offset += 1;
            }
        }
        new_char_mat[i] = new_row;
    }
    new_char_mat
}

fn get_galaxies_coords(char_mat: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies_coords: Vec<(usize, usize)> = vec![];
    for (i, row) in char_mat.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxies_coords.push((i, j));
            }
        }
    }
    galaxies_coords
}

fn get_galaxies_distances(galaxies_coords: Vec<(usize, usize)>) -> HashMap<((usize, usize), (usize, usize)), usize> {
    let mut distances: HashMap<((usize, usize), (usize, usize)), usize> = HashMap::new();
    for i in 0..galaxies_coords.len() {
        for j in (i + 1)..galaxies_coords.len() {
            let galaxy_coord_1 = galaxies_coords[i];
            let galaxy_coord_2 = galaxies_coords[j];
            let d = (galaxy_coord_1.0).abs_diff(galaxy_coord_2.0) + (galaxy_coord_1.1).abs_diff(galaxy_coord_2.1);
            distances.insert((galaxy_coord_1, galaxy_coord_2), d);
        }
    }
    distances
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
