use std::collections::HashMap;
use std::str::FromStr;
advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut read_stages = true;
    let mut stages_instructions = HashMap::new();
    let mut accepted_parts = vec![];
    for line in input.split("\n") {
        if line.len() > 0 {
            if read_stages {
                let (label, instructions) = parse_stages(line);
                stages_instructions.insert(label, instructions.clone());
            } else {
                let part = parse_part(line);
                if is_part_accepted(&stages_instructions, &part) {
                    accepted_parts.push(part);
                }
            }
        } else {
            // parse the variables after the first empty line
            read_stages = false;
        }
    }

    let mut sum = 0;
    for p in accepted_parts {
        for (_, val) in p {
            sum += val;
        }
    }

    Some(sum)
}

fn parse_stages(line: &str) -> (&str, Vec<(Option<(char, char, u32)>, &str)>) {
    let mut split = line.split("{");
    let label = split.nth(0).unwrap();
    let instructions_str = split.nth(0).unwrap().trim_end_matches("}");
    let mut instructions = vec![];
    for instruction in instructions_str.split(",") {
        let mut instruction_split = instruction.split(":");
        let condition = instruction_split.nth(0).unwrap();
        if instruction.contains(":") {
            let variable = char::from_str(&condition[0..1]).unwrap();
            let comparison = char::from_str(&condition[1..2]).unwrap();
            let value = condition[2..].parse::<u32>().unwrap();
            let destination = instruction_split.nth(0).unwrap();
            instructions.push((Some((variable, comparison, value)), destination));
        } else {
            instructions.push((None, condition));
        }
    }
    (label, instructions)
}

fn parse_part(line: &str) -> HashMap<char, u32> {
    let mut variables = HashMap::new();
    let trimmed_line = line.trim_start_matches("{").trim_end_matches("}");
    for v in trimmed_line.split(",") {
        let mut split = v.split("=");
        let name = char::from_str(split.nth(0).unwrap()).unwrap();
        let value = split.nth(0).unwrap().parse::<u32>().unwrap();
        variables.insert(name, value);
    }
    variables
}

fn is_part_accepted(stages_instructions: &HashMap<&str, Vec<(Option<(char, char, u32)>, &str)>>, part: &HashMap<char, u32>) -> bool {
    let mut stage = "in";
    while stage != "A" && stage != "R" {
        let instructions = &stages_instructions[stage];
        for instr in instructions {
            if instr.0.is_none() {
                stage = instr.1;
                break;
            } else {
                let (instr_var, instr_comp, instr_val) = instr.0.unwrap();
                let destination = instr.1;
                if instr_comp == '<' && part[&instr_var] < instr_val {
                    stage = destination;
                    break;
                } else if instr_comp == '>' && part[&instr_var] > instr_val {
                    stage = destination;
                    break;
                }
            }
        }
    }
    stage == "A"
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
