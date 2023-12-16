use tqdm::tqdm;
advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let char_mat = build_matrix(input);
    let start_coords = (0, 0);
    let start_going_direction = "right";

    let mut ray_path = vec![(start_coords, start_going_direction)];
    update_ray_path(&char_mat, start_coords, start_going_direction, &mut ray_path);
    // count unique tiles visited by the ray
    let mut energized_tiles = vec![];
    for tile in ray_path {
        let (coords, _) = tile;
        if !energized_tiles.contains(&coords) {
            energized_tiles.push(coords);
        }
    }
    /*
        for (i, row) in char_mat.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if energized_tiles.contains(&(i, j)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!();
        }
    */
    Some(energized_tiles.len())
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

/// Returns char and new coordinates after moving up
fn up(char_mat: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(char, usize, usize)> {
    if i > 0 {
        Some((char_mat[i - 1][j], i - 1, j))
    } else {
        None
    }
}

/// Returns char and new coordinates after moving down
fn down(char_mat: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(char, usize, usize)> {
    if i < (char_mat.len() - 1) {
        Some((char_mat[i + 1][j], i + 1, j))
    } else {
        None
    }
}

/// Returns char and new coordinates after moving right
fn right(char_mat: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(char, usize, usize)> {
    if j < (char_mat[0].len() - 1) {
        Some((char_mat[i][j + 1], i, j + 1))
    } else {
        None
    }
}

/// Returns char and new coordinates after moving left
fn left(char_mat: &Vec<Vec<char>>, i: usize, j: usize) -> Option<(char, usize, usize)> {
    if j > 0 {
        Some((char_mat[i][j - 1], i, j - 1))
    } else {
        None
    }
}

fn update_ray_path(
    char_mat: &Vec<Vec<char>>,
    start_coords: (usize, usize),
    start_going_direction: &'static str,
    ray_path: &mut Vec<((usize, usize), &'static str)>,
) {
    let mut coords = start_coords;
    let going_directions = get_going_directions(char_mat[coords.0][coords.1], start_going_direction);
    // update going direction
    let mut going_direction = going_directions[0];
    // if the current character is a splitter, recurse on the second direction
    if going_directions.len() > 1 {
        update_ray_path(char_mat, coords, going_directions[1], ray_path);
    }

    let mut move_ = match going_direction {
        "up" => up(char_mat, coords.0, coords.1),
        "down" => down(char_mat, coords.0, coords.1),
        "right" => right(char_mat, coords.0, coords.1),
        "left" => left(char_mat, coords.0, coords.1),
        _ => panic!("Invalid direction"),
    };
    let (mut c, mut new_i, mut new_j);
    if move_.is_none() {
        return;
    } else {
        (c, new_i, new_j) = move_.unwrap();
    }
    coords = (new_i, new_j);
    ray_path.push((coords, going_direction));

    loop {
        let going_directions = get_going_directions(c, going_direction);
        // update going direction
        going_direction = going_directions[0];
        // if the current character is a splitter, recurse on the second direction
        if going_directions.len() > 1 {
            update_ray_path(char_mat, coords, going_directions[1], ray_path);
        }
        move_ = match going_direction {
            "up" => up(char_mat, coords.0, coords.1),
            "down" => down(char_mat, coords.0, coords.1),
            "right" => right(char_mat, coords.0, coords.1),
            "left" => left(char_mat, coords.0, coords.1),
            _ => panic!("Invalid direction"),
        };
        if move_.is_none() {
            break;
        } else {
            (c, new_i, new_j) = move_.unwrap();
        }
        coords = (new_i, new_j);
        // stop iterating when starting to loop
        if ray_path.contains(&(coords, going_direction)) {
            break;
        } else {
            ray_path.push((coords, going_direction));
        }
    }
}

fn get_going_directions(c: char, going_direction: &str) -> Vec<&str> {
    match (c, going_direction) {
        ('.', _) => vec![going_direction],
        ('-', "right") => vec!["right"],
        ('-', "left") => vec!["left"],
        ('-', "up") => vec!["left", "right"],
        ('-', "down") => vec!["left", "right"],
        ('|', "right") => vec!["up", "down"],
        ('|', "left") => vec!["up", "down"],
        ('|', "up") => vec!["up"],
        ('|', "down") => vec!["down"],
        ('\\', "right") => vec!["down"],
        ('\\', "left") => vec!["up"],
        ('\\', "up") => vec!["left"],
        ('\\', "down") => vec!["right"],
        ('/', "right") => vec!["up"],
        ('/', "left") => vec!["down"],
        ('/', "up") => vec!["right"],
        ('/', "down") => vec!["left"],
        _ => panic!("No pipe here or starting position"),
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let char_mat = build_matrix(input);
    let mut energized_tiles_counts = vec![];

    for i in tqdm(0..char_mat.len()) {
        energized_tiles_counts.push(get_energized_tiles_count(&char_mat, (i, 0), "right"));
        energized_tiles_counts.push(get_energized_tiles_count(&char_mat, (i, char_mat[0].len() - 1), "left"));
    }

    for j in tqdm(0..char_mat[0].len()) {
        energized_tiles_counts.push(get_energized_tiles_count(&char_mat, (0, j), "down"));
        energized_tiles_counts.push(get_energized_tiles_count(&char_mat, (char_mat.len() - 1, j), "up"));
    }

    let mut max = None;
    for c in energized_tiles_counts {
        if max.is_none() {
            max = Some(c);
        } else if max.unwrap() < c {
            max = Some(c);
        }
    }

    max
}

fn get_energized_tiles_count(char_mat: &Vec<Vec<char>>, start_coords: (usize, usize), start_going_direction: &'static str) -> usize {
    let mut ray_path = vec![(start_coords, start_going_direction)];
    update_ray_path(&char_mat, start_coords, start_going_direction, &mut ray_path);
    // count unique tiles visited by the ray
    let mut energized_tiles = vec![];
    for tile in ray_path {
        let (coords, _) = tile;
        if !energized_tiles.contains(&coords) {
            energized_tiles.push(coords);
        }
    }
    energized_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
