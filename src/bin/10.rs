advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let char_mat = build_matrix(input);

    // find the coordinates of S
    let mut start_coords = (0, 0);
    for (i, char_vec) in char_mat.iter().enumerate() {
        for (j, c) in char_vec.iter().enumerate() {
            if *c == 'S' {
                start_coords = (i, j);
            }
        }
    }

    // Determine directions for distance calculation
    let start_going_direction;
    if ['|', '7', 'F'].contains(&(up(&char_mat, start_coords.0, start_coords.1).0)) {
        start_going_direction = "up";
    } else if ['|', 'L', 'J'].contains(&(down(&char_mat, start_coords.0, start_coords.1).0)) {
        start_going_direction = "down";
    } else if ['J', '7', '-'].contains(&(right(&char_mat, start_coords.0, start_coords.1).0)) {
        start_going_direction = "right";
    } else {
        start_going_direction = "left";
    }

    let loop_length = calculate_loop_length(&char_mat, start_coords, start_going_direction);

    Some(loop_length / 2)
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
fn up(char_mat: &Vec<Vec<char>>, i: usize, j: usize) -> (char, usize, usize) {
    (char_mat[i - 1][j], i - 1, j)
}

/// Returns char and new coordinates after moving down
fn down(char_mat: &Vec<Vec<char>>, i: usize, j: usize) -> (char, usize, usize) {
    (char_mat[i + 1][j], i + 1, j)
}

/// Returns char and new coordinates after moving right
fn right(char_mat: &Vec<Vec<char>>, i: usize, j: usize) -> (char, usize, usize) {
    (char_mat[i][j + 1], i, j + 1)
}

/// Returns char and new coordinates after moving left
fn left(char_mat: &Vec<Vec<char>>, i: usize, j: usize) -> (char, usize, usize) {
    (char_mat[i][j - 1], i, j - 1)
}

fn calculate_loop_length(char_mat: &Vec<Vec<char>>, start_coords: (usize, usize), start_going_direction: &str) -> u32 {
    let mut coords = start_coords;
    let mut coming_direction;
    let mut going_direction = start_going_direction;
    let mut length = 0;

    let (mut c, mut new_i, mut new_j) = match going_direction {
        "up" => up(char_mat, coords.0, coords.1),
        "down" => down(char_mat, coords.0, coords.1),
        "right" => right(char_mat, coords.0, coords.1),
        "left" => left(char_mat, coords.0, coords.1),
        _ => panic!("Invalid direction"),
    };
    // update coming direction
    coming_direction = match going_direction {
        "up" => "down",
        "down" => "up",
        "right" => "left",
        "left" => "right",
        _ => panic!("Invalid direction"),
    };
    coords = (new_i, new_j);
    length += 1;

    while c != 'S' {
        let directions = get_directions(c);
        // update going direction
        if directions.0 == coming_direction {
            going_direction = directions.1
        } else {
            going_direction = directions.0
        }
        (c, new_i, new_j) = match going_direction {
            "up" => up(char_mat, coords.0, coords.1),
            "down" => down(char_mat, coords.0, coords.1),
            "right" => right(char_mat, coords.0, coords.1),
            "left" => left(char_mat, coords.0, coords.1),
            _ => panic!("Invalid direction"),
        };
        // update coming direction
        coming_direction = match going_direction {
            "up" => "down",
            "down" => "up",
            "right" => "left",
            "left" => "right",
            _ => panic!("Invalid direction"),
        };
        coords = (new_i, new_j);
        length += 1;
    }
    length
}

fn get_directions(c: char) -> (&'static str, &'static str) {
    match c {
        '|' => ("up", "down"),
        '-' => ("left", "right"),
        'L' => ("up", "right"),
        'J' => ("up", "left"),
        '7' => ("down", "left"),
        'F' => ("down", "right"),
        _ => panic!("No pipe here or starting position"),
    }
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
