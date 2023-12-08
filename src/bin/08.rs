use std::collections::HashMap;
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_split = input.split("\n"); // Be careful to have input/example files with LF line ending
    let directions = input_split.nth(0).unwrap().chars();
    let mut nodes_edges: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut steps = 0;
    for line in input_split {
        if line.len() > 1 {
            let (position, new_positions) = parse_line(line);
            nodes_edges.insert(position, new_positions);
        }
    }
    let mut current_position = "AAA";
    let mut directions_mut = directions.clone();
    while current_position != "ZZZ" {
        let direction_opt = directions_mut.nth(0);
        let direction;
        if direction_opt.is_none() {
            directions_mut = directions.clone();
            direction = directions_mut.nth(0).unwrap();
        } else {
            direction = direction_opt.unwrap();
        }
        current_position = match direction {
            'L' => nodes_edges.get(current_position).unwrap().0,
            'R' => nodes_edges.get(current_position).unwrap().1,
            _ => panic!("Invalid direction: {}", direction),
        };
        steps += 1;
    }
    Some(steps)
}

fn parse_line(line: &str) -> (&str, (&str, &str)) {
    let mut line_split = line.split(" = ");
    let position = line_split.nth(0).unwrap();
    let mut new_positions_split = line_split.nth(0).unwrap().split(", ");
    let new_position_left = new_positions_split.nth(0).unwrap().trim_start_matches("(");
    let mut new_position_right = new_positions_split.nth(0).unwrap().trim_end_matches(")");
    (position, (new_position_left, new_position_right))
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
