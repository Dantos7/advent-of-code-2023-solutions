use qndr::get_numbers;
use rayon::prelude::*;
use tqdm::tqdm;
advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut prod:u32 = 1;
    let mut input_split = input.split("\n");
    let times = parse_vector(input_split.nth(0).unwrap(), "Time: ");
    let record_distances = parse_vector(input_split.nth(0).unwrap(), "Distance: ");
    for i in 0..times.len(){
        let winning_holding_times = get_winning_holding_times(times[i], record_distances[i]);
        prod *= winning_holding_times.len() as u32;
    }
    Some(prod)
}

fn parse_vector(line: &str, prefix: &str) -> Vec<u32>{
    line.strip_prefix(prefix).unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()
}

fn get_winning_holding_times(time:u32, record_distance:u32) -> Vec<u32> {
    let mut winning_holding_times = vec![];
    for holding_time in tqdm(1..time) {
        if ((time - holding_time) * holding_time) > record_distance {
            winning_holding_times.push(holding_time);
        }
    }
    winning_holding_times
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
