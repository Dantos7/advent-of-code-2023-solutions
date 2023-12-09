advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum = 0;
    let mut sequences: Vec<Vec<i32>> = vec![];
    for line in input.split("\n") {
        if line.len() > 0 {
            let sequence = parse_line(line);
            sequences.push(sequence);
        }
    }
    for sequence in sequences {
        let differences = get_differences(&sequence);
        let next_number = get_next_number_forward(differences);
        sum += next_number;
    }
    Some(sum)
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
}

fn get_differences(sequence: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut differences = vec![];
    let mut difference = (*sequence).clone();
    while !is_all_zeros(&difference) {
        differences.push(difference.clone());
        let mut new_difference = vec![0; difference.len() - 1];
        let len = difference.len();
        for i in (1..len).rev() {
            let diff = difference[i] - difference[i - 1];
            new_difference[i - 1] = diff;
        }
        difference = new_difference.clone();
    }
    differences
}

fn get_next_number_forward(mut differences: Vec<Vec<i32>>) -> i32 {
    // Compute the next value iteratively from the back
    let mut next_number = 0;
    for _ in 0..differences.len() {
        let difference = differences.pop().unwrap();
        next_number = difference.last().unwrap() + next_number;
    }
    next_number
}

fn is_all_zeros(vec: &Vec<i32>) -> bool {
    vec.iter().all(|&x| x == 0)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum = 0;
    let mut sequences: Vec<Vec<i32>> = vec![];
    for line in input.split("\n") {
        if line.len() > 0 {
            let sequence = parse_line(line);
            sequences.push(sequence);
        }
    }
    for sequence in sequences {
        let differences = get_differences(&sequence);
        let next_number = get_next_number_backwards(differences);
        sum += next_number;
    }
    Some(sum)
}

fn get_next_number_backwards(mut differences: Vec<Vec<i32>>) -> i32 {
    // Compute the next value iteratively from the back
    let mut next_number = 0;
    for _ in 0..differences.len() {
        let difference = differences.pop().unwrap();
        next_number = difference.first().unwrap() - next_number;
    }
    next_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
