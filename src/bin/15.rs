use std::collections::HashMap;
advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let line = input.split("\n").nth(0).unwrap();
    let steps = line.split(",");
    let mut sum: u32 = 0;
    for step in steps {
        sum += hash_algorithm(step);
    }
    Some(sum)
}

fn hash_algorithm(input: &str) -> u32 {
    /*
    To run the HASH algorithm on a string, start with a current value of 0. Then, for each character in the string starting from the beginning:
        Determine the ASCII code for the current character of the string.
        Increase the current value by the ASCII code you just determined.
        Set the current value to itself multiplied by 17.
        Set the current value to the remainder of dividing itself by 256.
    */
    let mut sum: u32 = 0;
    for c in input.chars() {
        sum += c as u32;
        sum *= 17;
        sum %= 256;
    }
    sum
}

pub fn part_two(input: &str) -> Option<u32> {
    let line = input.split("\n").nth(0).unwrap();
    let steps = line.split(",");
    let mut box_to_lenses: HashMap<u8, Vec<(&str, u32)>> = HashMap::new();
    for step in steps {
        if step.contains("-") {
            removal_op(step, &mut box_to_lenses)
        } else {
            addition_op(step, &mut box_to_lenses)
        }
    }
    let sum = compute_sum(box_to_lenses);
    Some(sum)
}

fn removal_op<'a>(step: &'a str, box_to_lenses: &mut HashMap<u8, Vec<(&'a str, u32)>>) {
    let label = step.split("-").nth(0).unwrap();
    let box_index = hash_algorithm(label) as u8;
    let lenses = box_to_lenses.get(&box_index);
    if lenses.is_none() {
        box_to_lenses.insert(box_index, vec![]);
    } else {
        let mut lenses_vec = lenses.unwrap().clone();
        for (i, (l, _)) in lenses_vec.iter().enumerate() {
            if label == *l {
                lenses_vec.remove(i);
                box_to_lenses.insert(box_index, lenses_vec);
                break;
            }
        }
    }
}

fn addition_op<'a>(step: &'a str, box_to_lenses: &mut HashMap<u8, Vec<(&'a str, u32)>>) {
    let mut step_split = step.split("=");
    let label = step_split.nth(0).unwrap();
    let focal_length = step_split.nth(0).unwrap().parse::<u32>().unwrap();
    let box_index = hash_algorithm(label) as u8;
    let lenses = box_to_lenses.get(&box_index);
    if lenses.is_none() {
        box_to_lenses.insert(box_index, vec![(label, focal_length)]);
    } else {
        let mut lenses_vec = lenses.unwrap().clone();
        for (i, (l, _)) in lenses_vec.iter().enumerate() {
            if label == *l {
                lenses_vec[i] = (*l, focal_length);
                box_to_lenses.insert(box_index, lenses_vec);
                return;
            }
        }
        // if the cycle didn't return, then the lens with label is not in the box already -> push it to the back
        lenses_vec.push((label, focal_length));
        box_to_lenses.insert(box_index, lenses_vec);
    }
}

fn compute_sum(box_to_lenses: HashMap<u8, Vec<(&str, u32)>>) -> u32 {
    let mut sum = 0;
    for (k, v) in box_to_lenses {
        let box_number: u32 = k as u32 + 1;
        for (i, (_, f)) in v.iter().enumerate() {
            let lens_position_number = i as u32 + 1;
            sum += box_number * lens_position_number * f;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
