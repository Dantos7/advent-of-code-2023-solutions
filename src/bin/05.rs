advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut read_ranges = false;
    let mut keys: Vec<u64> = vec![];
    let mut ranges: Vec<(u64, u64, u64)> = vec![];
    for line in input.split("\n") {
        if line.len() > 1 {
            if line.starts_with("seeds: ") {
                let line_split = line.strip_prefix("seeds: ").unwrap().split_whitespace();
                keys = line_split.map(|s| s.parse::<u64>().unwrap()).collect();
            } else if line.contains("map:") {
                read_ranges = true
            } else if read_ranges {
                let range = parse_ranged_list(line);
                ranges.push(range);
            }
        } else {
            if ranges.len() > 0 {
                let mut new_keys = keys.clone();
                for (i, k) in keys.iter().enumerate() {
                    new_keys[i] = get_new_key(&ranges, *k);
                }
                keys = new_keys.clone();
                ranges = vec![];
                read_ranges = false;
            }
        }
    }
    let min = *keys.iter().min().unwrap();
    Some(min)
}

fn parse_ranged_list(line: &str) -> (u64, u64, u64) {
    let mut split = line.split_whitespace();
    let destination_range_start = split.nth(0).unwrap().parse::<u64>().unwrap();
    let source_range_start = split.nth(0).unwrap().parse::<u64>().unwrap();
    let range_length = split.nth(0).unwrap().parse::<u64>().unwrap();
    (destination_range_start, source_range_start, range_length)
}

fn get_new_key(ranges: &Vec<(u64, u64, u64)>, key: u64) -> u64 {
    // use the same key if not found in range
    let mut new_key = key;
    for (destination_range_start, source_range_start, range_length) in ranges {
        if key > *source_range_start && key < (*source_range_start + *range_length) {
            new_key = key - source_range_start + destination_range_start;
            break;
        }
    }
    new_key
}

pub fn part_two(input: &str) -> Option<u64> {
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