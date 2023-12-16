advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let mut sum = 0;
    for line in input.split("\n") {
        if line.len() > 0 {
            let (sequence, contiguous_groups) = parse_line(line);
            let mut valid_combinations = vec![];
            let damaged_budget: usize = contiguous_groups.iter().sum::<usize>() - sequence.iter().filter(|&&c| c == '#').count();
            update_valid_combinations(&sequence, &contiguous_groups, &mut valid_combinations, damaged_budget);
            let combinations_count = valid_combinations.len();
            //println!("{:?}", valid_combinations);
            //println!("============== {combinations_count} =================");
            sum += combinations_count;
        }
    }
    Some(sum)
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    let mut line_split = line.split_whitespace();
    let sequence = line_split.nth(0).unwrap().chars().collect();
    let contiguous_groups: Vec<usize> = line_split
        .nth(0)
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    return (sequence, contiguous_groups);
}

fn update_valid_combinations(
    sequence: &Vec<char>,
    contiguous_groups: &Vec<usize>,
    valid_combinations: &mut Vec<Vec<char>>,
    damaged_budget: usize,
) {
    if !validate_sequence(sequence, contiguous_groups) {
        return;
    }

    let first_damaged_position = sequence.iter().position(|&r| r == '?');
    if first_damaged_position.is_none() {
        // if there are no damaged positions left, the sequence can be added
        valid_combinations.push(sequence.clone());
    } else {
        // Branch with good spring
        let mut new_sequence_good = sequence.clone();
        new_sequence_good[first_damaged_position.unwrap()] = '.';
        update_valid_combinations(&new_sequence_good, contiguous_groups, valid_combinations, damaged_budget);
        if damaged_budget > 0 {
            // Don't add new damaged position if all the damaged positions have been already
            let mut new_sequence_damaged = sequence.clone();
            new_sequence_damaged[first_damaged_position.unwrap()] = '#';
            update_valid_combinations(&new_sequence_damaged, contiguous_groups, valid_combinations, damaged_budget - 1);
        }
    }
}

fn validate_sequence(sequence: &Vec<char>, contiguous_groups: &Vec<usize>) -> bool {
    // println!("{:?}", sequence);
    let mut current_group_index = 0;
    let mut current_group_counter = 0;
    let mut counting = false;
    for c in sequence {
        // stop validating at the first ?
        if *c == '?' {
            return true;
        } else if *c == '#' {
            counting = true;
            current_group_counter += 1;
        } else {
            if counting {
                if current_group_index >= contiguous_groups.len() {
                    // too many groups
                    // println!("Too many groups {:?}", sequence);
                    return false;
                } else if current_group_counter != contiguous_groups[current_group_index] {
                    // too many/little contiguous positions in the group
                    // println!("Too many/little contiguous damaged positions in group {:?}", sequence);
                    return false;
                } else {
                    counting = false;
                    current_group_counter = 0;
                    current_group_index += 1;
                }
            }
        }
    }
    if counting {
        // println!("Final {:?}", sequence);
        return current_group_counter == contiguous_groups[current_group_index] && current_group_index == (contiguous_groups.len() - 1);
    } else {
        return current_group_index == contiguous_groups.len();
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
