use counter::Counter;
use std::cmp::Ordering;
advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands_values_bids: Vec<(&str, u32, u32, u32)> = vec![];
    for line in input.split("\n") {
        if line.len() > 0 {
            let (hand, bid) = parse_line(line);
            let hand_type_value = get_hand_type_value(&mut hand.chars().collect(), false);
            let numerical_hand = get_numerical_hand(hand, false);
            let hand_sequence_value = get_numerical_hand_sequence_value(&numerical_hand);
            hands_values_bids.push((hand, hand_type_value, hand_sequence_value, bid));
        }
    }
    hands_values_bids.sort_by(cmp_hand);
    let mut sum = 0;
    for (i, (hand, type_value, sequence_value, bid)) in hands_values_bids.iter().enumerate() {
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

fn get_numerical_hand(hand: &str, joker: bool) -> Vec<u32> {
    let mut numerical_hand = vec![];
    for c in hand.chars() {
        let number = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' if !joker => 11,
            'J' if joker => 1,
            'T' => 10,
            _ => c.to_digit(10).unwrap(),
        };
        numerical_hand.push(number);
    }
    numerical_hand
}

fn get_hand_type_value(hand_chars: &mut Vec<char>, joker: bool) -> u32 {
    if is_five_of_a_kind(hand_chars, joker) {
        return 7;
    } else if is_four_of_a_kind(hand_chars, joker) {
        return 6;
    } else if is_full_house(hand_chars, joker) {
        return 5;
    } else if is_three_of_a_kind(hand_chars, joker) {
        return 4;
    } else if is_two_pair(hand_chars, joker) {
        return 3;
    } else if is_one_pair(hand_chars, joker) {
        return 2;
    } else {
        return 1;
    }
}

fn is_five_of_a_kind(hand_chars: &mut Vec<char>, joker: bool) -> bool {
    let cards_counts = hand_chars.iter().map(|x| *x).collect::<Counter<char>>();
    let mut joker_count = 0;
    if joker {
        joker_count = match cards_counts.get(&'J') {
            Some(c) => *c,
            None => 0,
        };
    }
    for (card, count) in cards_counts {
        if count == 5 || (card != 'J' && ((count + joker_count) == 5)) {
            return true;
        }
    }
    false
}

fn is_four_of_a_kind(hand_chars: &mut Vec<char>, joker: bool) -> bool {
    let cards_counts = hand_chars.iter().map(|x| *x).collect::<Counter<char>>();
    let mut joker_count = 0;
    if joker {
        joker_count = match cards_counts.get(&'J') {
            Some(c) => *c,
            None => 0,
        };
    }
    for (card, count) in cards_counts {
        if count == 4 || (card != 'J' && ((count + joker_count) >= 4)) {
            return true;
        }
    }
    false
}

// TODO wrong
fn is_full_house(hand_chars: &mut Vec<char>, joker: bool) -> bool {
    let mut cards_counts = hand_chars.iter().map(|x| *x).collect::<Counter<char>>();
    let mut triple_found = false;
    let mut couple_found = false;
    let mut joker_count = 0;
    if joker {
        joker_count = match cards_counts.get(&'J') {
            Some(c) => *c,
            None => 0,
        };
    }
    // Search for a triple
    for (card, count) in cards_counts.clone() {
        if count == 3 {
            triple_found = true;
            cards_counts.remove(&card);
            break;
        }
    }
    // Do another round using 1 joker if didn't manage to find a triple
    let mut used_jokers = 0;
    while !triple_found && used_jokers < joker_count {
        used_jokers += 1;
        for (card, count) in cards_counts.clone() {
            // Don't consider the joker as a possible card to make a triple itself (already considered in the loop before
            if card != 'J' && (count + used_jokers) == 3 {
                triple_found = true;
                cards_counts.remove(&card);
                joker_count -= used_jokers;
                break;
            }
        }
    }
    cards_counts.insert('J', joker_count);
    // Search for a couple
    for (card, count) in cards_counts.clone() {
        if count == 2 {
            couple_found = true;
            cards_counts.remove(&card);
            break;
        }
    }
    used_jokers = 0;
    while !couple_found && used_jokers < joker_count {
        used_jokers += 1;
        for (card, count) in cards_counts.clone() {
            // Don't consider the joker as a possible card to make a triple itself (already considered in the loop before
            if card != 'J' && (count + used_jokers) == 2 {
                couple_found = true;
                cards_counts.remove(&card);
                joker_count -= used_jokers;
                break;
            }
        }
    }
    triple_found && couple_found
}

/// Correct only when called after is_full_house()
fn is_three_of_a_kind(hand_chars: &mut Vec<char>, joker: bool) -> bool {
    let cards_counts = hand_chars.iter().map(|x| *x).collect::<Counter<char>>();
    let mut joker_count = 0;
    if joker {
        joker_count = match cards_counts.get(&'J') {
            Some(c) => *c,
            None => 0,
        };
    }
    for (card, count) in cards_counts {
        if count == 3 || (card != 'J' && ((count + joker_count) >= 3)) {
            return true;
        }
    }
    false
}

/// Correct only when called after is_full_house()
fn is_two_pair(hand_chars: &mut Vec<char>, joker: bool) -> bool {
    let mut cards_counts = hand_chars.iter().map(|x| *x).collect::<Counter<char>>();
    let mut first_couple_found = false;
    let mut second_couple_found = false;
    let mut joker_count = 0;
    if joker {
        joker_count = match cards_counts.get(&'J') {
            Some(c) => *c,
            None => 0,
        };
    }
    // Search for the first couple
    for (card, count) in cards_counts.clone() {
        if count == 2 {
            first_couple_found = true;
            cards_counts.remove(&card);
            break;
        }
    }
    // Do another round using 1 joker if didn't manage to find a couple
    let mut used_jokers = 0;
    while !first_couple_found && used_jokers < joker_count {
        used_jokers += 1;
        for (card, count) in cards_counts.clone() {
            // Don't consider the joker as a possible card to make a triple itself (already considered in the loop before
            if card != 'J' && (count + used_jokers) == 2 {
                first_couple_found = true;
                cards_counts.remove(&card);
                joker_count -= used_jokers;
                break;
            }
        }
    }
    if !first_couple_found {
        // No need to look for the second couple if the first one is not found
        return false;
    }
    cards_counts.insert('J', joker_count);
    // Search for a couple
    for (card, count) in cards_counts.clone() {
        if count == 2 {
            second_couple_found = true;
            cards_counts.remove(&card);
            break;
        }
    }
    used_jokers = 0;
    while !second_couple_found && used_jokers < joker_count {
        used_jokers += 1;
        for (card, count) in cards_counts.clone() {
            // Don't consider the joker as a possible card to make a triple itself (already considered in the loop before
            if card != 'J' && (count + used_jokers) == 2 {
                second_couple_found = true;
                cards_counts.remove(&card);
                joker_count -= used_jokers;
                break;
            }
        }
    }
    first_couple_found && second_couple_found
}

/// Correct only when called after is_two_pair()
fn is_one_pair(hand_chars: &mut Vec<char>, joker: bool) -> bool {
    let cards_counts = hand_chars.iter().map(|x| *x).collect::<Counter<char>>();
    let mut joker_count = 0;
    if joker {
        joker_count = match cards_counts.get(&'J') {
            Some(c) => *c,
            None => 0,
        };
    }
    for (card, count) in cards_counts {
        if count == 2 || (card != 'J' && ((count + joker_count) >= 2)) {
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
    let mut hands_values_bids: Vec<(&str, u32, u32, u32)> = vec![];
    for line in input.split("\n") {
        if line.len() > 0 {
            let (hand, bid) = parse_line(line);
            let hand_type_value = get_hand_type_value(&mut hand.chars().collect(), true);
            let numerical_hand = get_numerical_hand(hand, true);
            let hand_sequence_value = get_numerical_hand_sequence_value(&numerical_hand);
            hands_values_bids.push((hand, hand_type_value, hand_sequence_value, bid));
        }
    }
    hands_values_bids.sort_by(cmp_hand);
    let mut sum = 0;
    for (i, (hand, type_value, sequence_value, bid)) in hands_values_bids.iter().enumerate() {
        sum += (i as u32 + 1) * bid;
    }
    Some(sum)
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
        assert_eq!(result, Some(5905));
    }
}
