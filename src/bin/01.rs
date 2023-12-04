advent_of_code::solution!(1);

/// Solver for part one of the current day
pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.split("\n") {
        if line.len() > 0 {
            sum += get_line_number_numeric(line)?;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.split("\n") {
        if line.len() > 0 {
            sum += get_line_number_alphanumeric(line)?;
        }
    }
    Some(sum)
}

/// Returns the number associated to the line by combining the first and last digit
pub fn get_line_number_numeric(input: &str) -> Option<u32> {
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

/// Returns the number associated to the line by combining the first and last digit
/// In this second version, numbers can also be spelled in letters
/// ! Replacing spelled digits with actual digits does not work in cases where names are concatenated (e.g. twone)
pub fn get_line_number_alphanumeric(input: &str) -> Option<u32> {
    let first_digit = get_first_alphanumeric_digit(input)?;
    let last_digit = get_last_alphanumeric_digit(input)?;
    let number = Some(format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap());
    number
}

fn get_first_alphanumeric_digit(input: &str) -> Option<String> {
    let mut first_digit = None;
    let mut spelled_number_run = "".to_owned();
    for c in input.chars() {
        spelled_number_run = spelled_number_run + &c.to_string();
        if let Some(_) = c.to_digit(10) {
            spelled_number_run = String::from("");
            if first_digit.is_none() {
                first_digit = Some(c.to_string());
                break;
            }
        } else {
            if let Some(d) = get_spelled_digit_forward(&spelled_number_run) {
                spelled_number_run = String::from("");
                if first_digit.is_none() {
                    first_digit = Some(d.clone());
                    break;
                }
            }
        }
    }
    first_digit
}

fn get_spelled_digit_forward(input: &str) -> Option<String> {
    let digit;
    let mut found_spelled_digit = None;
    let vocabulary_spelled_digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for spelled_digit in vocabulary_spelled_digits.into_iter() {
        if input.contains(spelled_digit) {
            found_spelled_digit = Some(spelled_digit);
            break;
        }
    }

    if found_spelled_digit.is_none() {
        digit = None;
    } else {
        digit = match found_spelled_digit? {
            "one" => Some(String::from("1")),
            "two" => Some(String::from("2")),
            "three" => Some(String::from("3")),
            "four" => Some(String::from("4")),
            "five" => Some(String::from("5")),
            "six" => Some(String::from("6")),
            "seven" => Some(String::from("7")),
            "eight" => Some(String::from("8")),
            "nine" => Some(String::from("9")),
            _ => None,
        };
    }
    digit
}

fn get_last_alphanumeric_digit(input: &str) -> Option<String> {
    // Since we are now searching backwards, the search can halt at the first found digit
    let new_input = input.chars().rev().collect::<String>();
    let mut last_digit = None;
    let mut spelled_number_run = "".to_owned();
    for c in new_input.chars() {
        spelled_number_run = spelled_number_run + &c.to_string();
        if let Some(_) = c.to_digit(10) {
            spelled_number_run = String::from("");
            if last_digit.is_none() {
                last_digit = Some(c.to_string());
                break;
            }
        } else {
            if let Some(d) = get_spelled_digit_backwards(&spelled_number_run) {
                spelled_number_run = String::from("");
                if last_digit.is_none() {
                    last_digit = Some(d.clone());
                    break;
                }
            }
        }
    }
    last_digit
}

fn get_spelled_digit_backwards(input: &str) -> Option<String> {
    let digit;
    let mut found_spelled_digit = None;
    let vocabulary_spelled_digits_backwards = ["eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];
    for spelled_digit in vocabulary_spelled_digits_backwards.into_iter() {
        if input.contains(spelled_digit) {
            found_spelled_digit = Some(spelled_digit);
            break;
        }
    }

    if found_spelled_digit.is_none() {
        digit = None;
    } else {
        digit = match found_spelled_digit? {
            "eno" => Some(String::from("1")),
            "owt" => Some(String::from("2")),
            "eerht" => Some(String::from("3")),
            "ruof" => Some(String::from("4")),
            "evif" => Some(String::from("5")),
            "xis" => Some(String::from("6")),
            "neves" => Some(String::from("7")),
            "thgie" => Some(String::from("8")),
            "enin" => Some(String::from("9")),
            _ => None,
        };
    }
    digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(583));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(617));
    }
}
