use qndr::get_numbers;
use tqdm::tqdm;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut prod: u64 = 1;
    let mut input_split = input.split("\n");
    let times = parse_vector(input_split.nth(0).unwrap(), "Time: ");
    let record_distances = parse_vector(input_split.nth(0).unwrap(), "Distance: ");
    for i in 0..times.len() {
        let winning_holding_times = get_winning_holding_times(times[i], record_distances[i]);
        prod *= winning_holding_times.len() as u64;
    }
    Some(prod)
}

fn parse_vector(line: &str, prefix: &str) -> Vec<u64> {
    line.strip_prefix(prefix)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn get_winning_holding_times(time: u64, record_distance: u64) -> Vec<u64> {
    let mut winning_holding_times = vec![];
    for holding_time in tqdm(1..time) {
        if ((time - holding_time) * holding_time) > record_distance {
            winning_holding_times.push(holding_time);
        }
    }
    winning_holding_times
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input_split = input.split("\n");
    let time = parse_int(input_split.nth(0).unwrap());
    let record_distance = parse_int(input_split.nth(0).unwrap());
    let winning_holding_times = get_winning_holding_times(time, record_distance);
    Some(winning_holding_times.len() as u64)
}

/// Parse int, ignoring other non-numeric chars in the string
fn parse_int(input: &str) -> u64 {
    get_numbers(input).parse::<u64>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
