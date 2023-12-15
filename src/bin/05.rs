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
    let mut read_ranges = false;
    let mut ranged_keys: Vec<(u64, u64)> = vec![];
    let mut ranges: Vec<(u64, u64, u64)> = vec![];
    for line in input.split("\n") {
        if line.len() > 1 {
            if line.starts_with("seeds: ") {
                let line_split = line.strip_prefix("seeds: ").unwrap().split_whitespace();
                let mut ranged_key: (u64, u64) = (0, 0);
                let mut counter = 0;
                for val in line_split.map(|s| s.parse::<u64>().unwrap()) {
                    if counter == 0 {
                        ranged_key.0 = val;
                        counter += 1;
                    } else {
                        ranged_key.1 = val;
                        ranged_keys.push(ranged_key);
                        ranged_key = (0, 0);
                        counter = 0;
                    }
                }
            } else if line.contains("map:") {
                read_ranges = true
            } else if read_ranges {
                let range = parse_ranged_list(line);
                ranges.push(range);
            }
        } else {
            if ranges.len() > 0 {
                let mut new_ranged_keys = vec![];
                for k in ranged_keys.iter() {
                    new_ranged_keys.append(&mut get_new_ranged_keys(&ranges, *k));
                }
                ranged_keys = new_ranged_keys.clone();
                ranges = vec![];
                read_ranges = false;
            }
        }
    }
    // determine minimum final number in the ranged_keys
    let mut min = None;
    for (start, _) in ranged_keys {
        if min.is_none() {
            min = Some(start);
        } else if min.unwrap() > start {
            min = Some(start);
        }
    }
    min
}

fn get_new_ranged_keys(ranges: &Vec<(u64, u64, u64)>, ranged_key: (u64, u64)) -> Vec<(u64, u64)> {
    // use the same key if not found in range
    let mut new_ranged_keys = vec![];
    let ranged_key_start = ranged_key.0;
    let ranged_key_range = ranged_key.1;
    let ranged_key_end = ranged_key_start + ranged_key_range - 1;
    for (destination_range_start_ref, source_range_start_ref, range_length_ref) in ranges {
        // Check range intersection (rk = ranged_key, r = range)
        // 1. rk doesn't intersect r -> continue (if no range intersects the key, return the exact range)
        // 2. rk is completely contained in r -> return mapped_rk
        // 3. r is completely contained in rk -> return mapped_r, call get_new_ranged_keys on the two separate intersections
        // 4. rk intersect partially r (intersection is on the right side of rk) -> call get_new_ranged_keys on the intersection + return mapped_intersection
        // 5. rk intersect partially r (intersection is on the left side of rk) -> call get_new_ranged_keys on the intersection + return mapped_intersection

        // the function returns a minimum of 1 and a maximum of 3 ranged keys

        let source_range_start = *source_range_start_ref;
        let range_length = *range_length_ref;
        let source_range_end = source_range_start + range_length - 1;
        let destination_range_start = *destination_range_start_ref;
        let destination_range_end = destination_range_start + range_length - 1;
        // Calculate the intersection in each case
        if (ranged_key_start >= source_range_start) && (ranged_key_end <= source_range_end) {
            // 2. rk is completely contained in r
            let mapped_ranged_key_start = ranged_key_start - source_range_start + destination_range_start;
            new_ranged_keys.push((mapped_ranged_key_start, ranged_key_range));
            return new_ranged_keys;
        } else if (ranged_key_start < source_range_start) && (ranged_key_end > source_range_end) {
            // 3. r is completely contained in rk
            new_ranged_keys.push((destination_range_start, range_length));
            new_ranged_keys.append(&mut get_new_ranged_keys(
                ranges,
                (source_range_end, ranged_key_end - source_range_end),
            )); // right-difference
            new_ranged_keys.append(&mut get_new_ranged_keys(
                ranges,
                (ranged_key_start, source_range_start - ranged_key_start),
            )); // left difference
            return new_ranged_keys;
        } else if (ranged_key_start < source_range_start) && (ranged_key_end < source_range_end) && (ranged_key_end > source_range_start) {
            // 4. rk intersect partially r (intersection is on the right side of rk)
            let mapped_ranged_key_end = ranged_key_end - source_range_start + destination_range_start;
            new_ranged_keys.push((destination_range_start, mapped_ranged_key_end - destination_range_start));
            new_ranged_keys.append(&mut get_new_ranged_keys(
                ranges,
                (ranged_key_start, source_range_start - ranged_key_start),
            ));
            return new_ranged_keys;
        } else if (ranged_key_start > source_range_start) && (ranged_key_start < source_range_end) && (ranged_key_end > source_range_end) {
            // 5. rk intersect partially r (intersection is on the left side of rk)
            let mapped_ranged_key_start = ranged_key_start - source_range_start + destination_range_start;
            new_ranged_keys.push((mapped_ranged_key_start, destination_range_end - mapped_ranged_key_start));
            new_ranged_keys.append(&mut get_new_ranged_keys(
                ranges,
                (source_range_end, ranged_key_end - source_range_end),
            ));
            return new_ranged_keys;
        } else {
            // No intersection
        }
    }
    vec![(ranged_key_start, ranged_key_range)]
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
        assert_eq!(result, Some(46));
    }
}
