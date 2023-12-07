use std::collections::HashMap;
use tqdm::tqdm;
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut read_ranges = false;
    let mut keys: Vec<u32> = vec![];
    let mut hash_map: HashMap<u32, u32> = HashMap::new();
    for line in input.split("\n") {
        if line.len() > 1 {
            if line.starts_with("seeds: ") {
                let line_split = line.strip_prefix("seeds: ").unwrap().split_whitespace();
                keys = line_split.map(|s| s.parse::<u32>().unwrap()).collect();
            } else if line.contains("map:") {
                read_ranges = true
            } else if read_ranges {
                let (destination_range_start, source_range_start, range_length) = parse_ranged_list(line);
                populate_hash_map(&mut hash_map, destination_range_start, source_range_start, range_length);
            }
        } else {
            if read_ranges {
                let mut new_keys = keys.clone();
                for (i, k) in keys.iter().enumerate() {
                    new_keys[i] = match hash_map.get(k) {
                        Some(new_k) => *new_k,
                        None => *k,
                    };
                }
                keys = new_keys;
                hash_map = HashMap::new();
                read_ranges = false;
            }
        }
    }
    let min = *keys.iter().min().unwrap();
    Some(min)
}

fn parse_ranged_list(line: &str) -> (u32, u32, u32) {
    let mut split = line.split_whitespace();
    let destination_range_start = split.nth(0).unwrap().parse::<u32>().unwrap();
    let source_range_start = split.nth(0).unwrap().parse::<u32>().unwrap();
    let range_length = split.nth(0).unwrap().parse::<u32>().unwrap();
    (destination_range_start, source_range_start, range_length)
}

fn populate_hash_map(hash_map: &mut HashMap<u32, u32>, destination_range_start: u32, source_range_start: u32, range_length: u32) {
    for i in tqdm(0..(range_length)) {
        hash_map.insert(source_range_start + i, destination_range_start + i);
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
