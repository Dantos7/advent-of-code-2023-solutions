use qndr::get_numbers;
use std::collections::HashMap;
use tqdm::tqdm;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.split("\n") {
        if line.len() > 0 {
            let (_, winning_numbers, own_numbers) = parse_card(line);
            sum += get_score_of_card(winning_numbers, own_numbers);
        }
    }
    Some(sum)
}

fn parse_card(line: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let mut split_iter = line.split(": ");
    let card_id = parse_int(split_iter.nth(0).unwrap()).unwrap();
    let mut numbers_split_iter = split_iter.nth(0).unwrap().split(" | ");
    let winning_numbers_split = numbers_split_iter.nth(0).unwrap().split_whitespace();
    let winning_numbers: Vec<u32> = winning_numbers_split.map(|s| parse_int(s).unwrap()).collect();
    let own_numbers_split = numbers_split_iter.nth(0).unwrap().split_whitespace();
    let own_numbers: Vec<u32> = own_numbers_split.map(|s| parse_int(s).unwrap()).collect();

    (card_id, winning_numbers, own_numbers)
}

fn get_score_of_card(winning_numbers: Vec<u32>, own_numbers: Vec<u32>) -> u32 {
    let mut score = 0;
    for wn in winning_numbers.iter() {
        if own_numbers.contains(wn) {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
    }
    score
}

/// Parse int, ignoring other non-numeric chars in the string
fn parse_int(input: &str) -> Option<u32> {
    Some(get_numbers(input).parse::<u32>().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut cards_count: HashMap<u32, u32> = HashMap::new();
    for (row, line) in tqdm(input.split("\n").enumerate()) {
        if line.len() > 0 {
            let card_id = row as u32 + 1;
            match cards_count.get(&card_id) {
                Some(c) => cards_count.insert(card_id, *c),
                None => cards_count.insert(card_id, 1),
            };
            let card_count = *cards_count.get(&card_id).unwrap();
            for _copy in 0..card_count {
                let (card_id, winning_numbers, own_numbers) = parse_card(line);
                let matches = get_number_of_matches(winning_numbers, own_numbers);
                for i in (card_id + 1)..(card_id + matches + 1) {
                    match cards_count.get(&i) {
                        Some(c) => cards_count.insert(i, c + 1),
                        None => cards_count.insert(i, 2),
                    };
                }
            }
        }
    }
    for (_, v) in cards_count.iter() {
        sum += v;
    }
    Some(sum)
}

fn get_number_of_matches(winning_numbers: Vec<u32>, own_numbers: Vec<u32>) -> u32 {
    let mut matches = 0;
    for wn in winning_numbers.iter() {
        if own_numbers.contains(wn) {
            matches += 1;
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
