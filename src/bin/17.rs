use std::process::exit;
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = build_matrix(input);
    let path= vec![(0,0)];
    let path_cost = 0;
    let mut previous_direction;
    let mut previous_direction_counter;

    // Compute a naive solution to start
    let mut minimum_cost = 1400;
    let mut minimum_path = vec![];

    // go right
    let (cost,i,j) = right(&matrix, 0,0).unwrap();
    let mut right_path = path.clone();
    let right_path_cost = path_cost + cost;
    right_path.push((i,j));
    previous_direction = "right";
    previous_direction_counter = 1;
    update_paths(&matrix, right_path, right_path_cost, previous_direction, previous_direction_counter, &mut minimum_path, &mut minimum_cost);

    // go down
    let (cost,i,j) = down(&matrix, 0,0).unwrap();
    let mut down_path = path.clone();
    let down_path_cost = path_cost + cost;
    down_path.push((i,j));
    previous_direction = "down";
    previous_direction_counter = 1;
    update_paths(&matrix, down_path, down_path_cost, previous_direction, previous_direction_counter, &mut minimum_path, &mut minimum_cost);

    Some(minimum_cost)
}

fn build_matrix(input: &str) -> Vec<Vec<u32>> {
    let mut matrix = vec![];
    for line in input.split("\n") {
        if line.len() > 0 {
            let cost_vec= line.chars().map(|x| x.to_digit(10).unwrap()).collect();
            matrix.push(cost_vec);
        }
    }
    matrix
}

/// Returns char and new coordinates after moving up
fn up(mat: &Vec<Vec<u32>>, i: usize, j: usize) -> Option<(u32, usize, usize)> {
    if i > 0 {
        Some((mat[i - 1][j], i - 1, j))
    } else {
        None
    }
}

/// Returns char and new coordinates after moving down
fn down(mat: &Vec<Vec<u32>>, i: usize, j: usize) -> Option<(u32, usize, usize)> {
    if i < (mat.len() - 1) {
        Some((mat[i + 1][j], i + 1, j))
    } else {
        None
    }
}

/// Returns char and new coordinates after moving right
fn right(mat: &Vec<Vec<u32>>, i: usize, j: usize) -> Option<(u32, usize, usize)> {
    if j < (mat[0].len() - 1) {
        Some((mat[i][j + 1], i, j + 1))
    } else {
        None
    }
}

/// Returns char and new coordinates after moving left
fn left(mat: &Vec<Vec<u32>>, i: usize, j: usize) -> Option<(u32, usize, usize)> {
    if j > 0 {
        Some((mat[i][j - 1], i, j - 1))
    } else {
        None
    }
}


fn update_paths(matrix: &Vec<Vec<u32>>, path:Vec<(usize,usize)>, path_cost:u32, previous_direction:&str, previous_direction_counter:u8, minimum_path: &mut Vec<(usize,usize)>, minimum_cost: &mut u32) {
    let last_cell = path.last().unwrap();
    let mut new_previous_direction_counter;

    if path.len() > 30 {
        return
    }

    for direction in ["right", "down", "left", "up"] {
        if direction == previous_direction {
            if previous_direction_counter == 3 {
                // 3 consecutive moves reached -> can not continue in this direction
                continue
            } else {
                new_previous_direction_counter = previous_direction_counter + 1;
            }
        } else {
            new_previous_direction_counter = 1;
        }

        let move_ = match direction {
            "left" => left(&matrix, last_cell.0, last_cell.1),
            "right" => right(&matrix, last_cell.0, last_cell.1),
            "down" => down(&matrix, last_cell.0, last_cell.1),
            "up" => up(&matrix, last_cell.0, last_cell.1),
            _ => panic!("Invalid direction"),
        };
        if move_.is_none() {
            // Out of matrix -> do nothing and return
            // println!("Out of board");
            continue
        } else {
            let (cost, i, j) = move_.unwrap();
            let new_cost = path_cost + cost;
            if path.contains(&(i, j)) {
                // Looping -> do nothing and return
                // println!("Looping {i} {j}");
                continue
            } else if *minimum_cost < new_cost {
                // Sub-optimal path -> do nothing and return
                // println!("Sub-optimal");
                continue
            } else if (i != matrix.len() - 1) || (j != matrix[0].len() - 1) {
                // Not reached the right-bottom corner yet -> add cell and continue
                let mut new_path = path.clone();
                new_path.push((i,j));
                update_paths(&matrix, new_path, new_cost, direction, new_previous_direction_counter, minimum_path, minimum_cost);
            } else {
                // Reached the right-bottom corner -> if cost is less than minimum update minimum path
                if new_cost < *minimum_cost {
                    println!("New minimum {new_cost} {}", path.len());
                    let mut new_path = path.clone();
                    new_path.push((i,j));
                    *minimum_cost = new_cost;
                    *minimum_path = new_path;
                }
                return
            }
        }
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
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
