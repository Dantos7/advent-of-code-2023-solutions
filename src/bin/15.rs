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
    None
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
        assert_eq!(result, None);
    }
}
