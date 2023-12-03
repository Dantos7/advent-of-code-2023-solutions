use std::collections::HashMap;
use qndr::get_numbers;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum=0;
    const RED:u32 = 12;
    const GREEN:u32 = 13;
    const BLUE:u32 = 14;

    for line in input.split("\n") {
        if line.len() > 0 {
            let Some((game_id, sets)) = parse_line(line) else { panic!() };
            if is_game_valid(sets, RED, GREEN, BLUE) {
                sum += game_id;
            }
        }
    }
    Some(sum)
}

fn parse_line(line: &str) -> Option<(u32, Vec<HashMap<&str,u32>>)>{
    let mut sets:Vec<HashMap<&str,u32>> = vec![];
    let mut line_split_1 = line.split(":");
    let game_id = parse_int(line_split_1.nth(0)?)?; // ! nth() consumes the iterator
    for set_split in line_split_1.nth(0)?.split(";") {
        let mut set = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);
        for color_split in set_split.split(",") {
            for (color, _) in set.clone().into_iter(){
                if color_split.contains(color) {
                    set.insert(color, parse_int(color_split)?);
                }
            }
        }
        sets.push(set);
    }
    Some((game_id, sets))
}


fn is_game_valid(sets:Vec<HashMap<&str,u32>>, red:u32, green:u32, blue:u32) -> bool {
    for set in sets.into_iter(){
        if set["red"] > red || set["green"] > green ||  set["blue"] > blue {
            return false
        }
    }
    true
}


/// Parse int, ignoring other non-numeric chars in the string
fn parse_int(input: &str) -> Option<u32> {
    Some(get_numbers(input).parse::<u32>().unwrap())
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
