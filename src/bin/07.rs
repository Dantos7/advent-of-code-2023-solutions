use counter::Counter;
use std::cmp::Ordering;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands_values_bids: Vec<(&str, u32, u32, u32)> = vec![];
    for line in input.split("\n") {
        if line.len() > 0 {
            let (hand, bid) = parse_line(line);
            let numerical_hand = get_numerical_hand(hand);
            let hand_type_value = get_numerical_hand_type_value(&numerical_hand);
            let hand_sequence_value = get_numerical_hand_sequence_value(&numerical_hand);
            hands_values_bids.push((hand, hand_type_value, hand_sequence_value, bid));
        }
    }
    hands_values_bids.sort_by(cmp_hand);
    let mut sum = 0;
    for (i, (hand, type_value, sequence_value, bid)) in hands_values_bids.iter().enumerate() {
        // println!("{} {} {} {}", hand, type_value, sequence_value, bid);
        sum += (i as u32 + 1) * bid;
    }
    Some(sum)
}

fn parse_line(line: &str) -> (&str, u32) {
    let mut split = line.split_whitespace();
    let hand = split.nth(0).unwrap();
    let bid = split.nth(0).unwrap().parse::<u32>().unwrap();
    (hand, bid)
}

fn get_numerical_hand(hand: &str) -> Vec<u32> {
    let mut numerical_hand = vec![];
    for c in hand.chars() {
        let number = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        };
        numerical_hand.push(number);
    }
    numerical_hand
}

fn get_numerical_hand_type_value(numerical_hand: &Vec<u32>) -> u32 {
    if is_five_of_a_kind(numerical_hand) {
        return 7;
    } else if is_four_of_a_kind(numerical_hand) {
        return 6;
    } else if is_full_house(numerical_hand) {
        return 5;
    } else if is_three_of_a_kind(numerical_hand) {
        return 4;
    } else if is_two_pair(numerical_hand) {
        return 3;
    } else if is_one_pair(numerical_hand) {
        return 2;
    } else {
        return 1;
    }
}

fn is_five_of_a_kind(numerical_hand: &Vec<u32>) -> bool {
    let cards_counts = numerical_hand.iter().map(|x| *x).collect::<Counter<u32>>();
    cards_counts.len() == 1
}

fn is_four_of_a_kind(numerical_hand: &Vec<u32>) -> bool {
    let cards_counts = numerical_hand.iter().map(|x| *x).collect::<Counter<u32>>();
    for (_, count) in cards_counts {
        if count == 4 {
            return true;
        }
    }
    false
}

fn is_full_house(numerical_hand: &Vec<u32>) -> bool {
    let cards_counts = numerical_hand.iter().map(|x| *x).collect::<Counter<u32>>();
    let mut triple_found = false;
    let mut couple_found = false;
    for (_, count) in cards_counts {
        if count == 3 {
            triple_found = true;
        } else if count == 2 {
            couple_found = true;
        }
    }
    triple_found && couple_found
}

/// Correct only when called after is_full_house()
fn is_three_of_a_kind(numerical_hand: &Vec<u32>) -> bool {
    let cards_counts = numerical_hand.iter().map(|x| *x).collect::<Counter<u32>>();
    for (_, count) in cards_counts {
        if count == 3 {
            return true;
        }
    }
    false
}

/// Correct only when called after is_full_house()
fn is_two_pair(numerical_hand: &Vec<u32>) -> bool {
    let cards_counts = numerical_hand.iter().map(|x| *x).collect::<Counter<u32>>();
    let mut first_pair_found = false;
    let mut second_pair_found = false;
    for (_, count) in cards_counts {
        if count == 2 && !first_pair_found {
            first_pair_found = true;
        } else if count == 2 && first_pair_found {
            second_pair_found = true;
        }
    }
    first_pair_found && second_pair_found
}

/// Correct only when called after is_two_pair()
fn is_one_pair(numerical_hand: &Vec<u32>) -> bool {
    let cards_counts = numerical_hand.iter().map(|x| *x).collect::<Counter<u32>>();
    for (_, count) in cards_counts {
        if count == 2 {
            return true;
        }
    }
    false
}

fn get_numerical_hand_sequence_value(numerical_hand: &Vec<u32>) -> u32 {
    numerical_hand[0] * 15u32.pow(4)
        + numerical_hand[1] * 15u32.pow(3)
        + numerical_hand[2] * 15u32.pow(2)
        + numerical_hand[3] * 15u32
        + numerical_hand[4]
}

fn cmp_hand(a: &(&str, u32, u32, u32), b: &(&str, u32, u32, u32)) -> Ordering {
    if a.1 < b.1 {
        return Ordering::Less;
    } else if a.1 > b.1 {
        return Ordering::Greater;
    } else {
        // a == b
        if a.2 < b.2 {
            return Ordering::Less;
        } else if a.2 > b.2 {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
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
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
