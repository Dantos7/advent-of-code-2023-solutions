advent_of_code::solution!(1);

/// Solver for part one of the current day
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum=0;

    for line in input.split("\n") {
        if line.len() > 0 {
            sum += get_line_number_one(line)?;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

/// Returns the number associated to the line by combining the first and last digit
pub fn get_line_number_one(input: &str) -> Option<u32> {
    let mut first_digit = None;
    let mut last_digit = None;
    for c in input.chars() {
        if let Some(_) = c.to_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(c.to_string());
            }
            last_digit = Some(c.to_string());
        }
    }
    let number = Some(format!("{}{}", first_digit?, last_digit?).parse::<u32>().unwrap());
    number
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(434));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(518));
    }
}
