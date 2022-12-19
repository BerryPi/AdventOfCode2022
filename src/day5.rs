use itertools::Itertools;
use regex::Regex;


fn add_label_to_stack(stacks: &mut Vec<Vec<char>>, label: char, stack_idx: usize) {
    while stacks.len() <= stack_idx {
        stacks.push(Vec::new());
    }
    // Since we're reading the stacks top down, add new labels to the front
    stacks[stack_idx].insert(0, label);
}

fn parse_stacks(input: &[String]) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in input.iter() {
        let mut line_iter = line.chars();
        let mut stack_idx: usize = 0;

        loop {
            let crate_start = line_iter.next();
            // Either this is a space, in which case there is no crate
            // and we move on to the next stack, 
            // this is a [ and the next character is a label,            
            // or we have reached the end of the line.
            if crate_start == Some(' ') {
                line_iter.next();
                line_iter.next();
                line_iter.next();
            }
            else if crate_start == Some('[') {
                let label = line_iter.next().unwrap();
                add_label_to_stack(&mut stacks, label, stack_idx);
                line_iter.next();
                line_iter.next();
            }
            else if crate_start == None {
                break;
            }
            stack_idx += 1;
        }
    }

    return stacks;
}

fn parse_instruction(inst: &String) -> (usize, usize, i32) {
    let inst_regex = Regex::new(r"move (?P<qty>\d*) from (?P<src>\d*) to (?P<dst>\d*)").unwrap();
    let caps = inst_regex.captures(inst).unwrap();

    let qty = caps.name("qty").unwrap().as_str().parse::<i32>().unwrap();
    // Stacks are 1-indexed in the input
    let src = caps.name("src").unwrap().as_str().parse::<usize>().unwrap() - 1;
    let dst = caps.name("dst").unwrap().as_str().parse::<usize>().unwrap() - 1;

    return (src, dst, qty);
}

fn apply_instruction(stacks: &mut Vec<Vec<char>>, instr: (usize, usize, i32)) {
    let qty = instr.2;
    let src = instr.0;
    let dst = instr.1;

    for _i in 0..qty {
        let label = stacks[src].pop().unwrap();
        stacks[dst].push(label);
    }
}

pub fn part1(input: Vec<String>) -> String {
    // Find where the crate stacks end - this will be the line
    // beginning with " 1"
    let crates_end = input.iter().find_position(|line| line.starts_with(" 1")).unwrap().0;
    let mut crates = parse_stacks(&input[..crates_end]);

    // After a blank line, the remainder is instructions
    let _ = &input[crates_end + 2..].iter()
        .map(parse_instruction)
        .for_each(|inst| apply_instruction(&mut crates, inst));

    let res = crates.iter()
        .map(|s| s[s.len() - 1]);
    return String::from_iter(res);
}

fn apply_instruction_v2(stacks: &mut Vec<Vec<char>>, instr: (usize, usize, i32)) {
    let qty = instr.2;
    let src = instr.0;
    let dst = instr.1;

    // To preserve order, make a temporary holding stack
    let mut holding: Vec<char> = Vec::new();
    for _i in 0..qty {
        let label = stacks[src].pop().unwrap();
        holding.push(label);
    }
    for _i in 0..qty {
        let label = holding.pop().unwrap();
        stacks[dst].push(label);
    }
}

pub fn part2(input: Vec<String>) -> String {
    // Find where the crate stacks end - this will be the line
    // beginning with " 1"
    let crates_end = input.iter().find_position(|line| line.starts_with(" 1")).unwrap().0;
    let mut crates = parse_stacks(&input[..crates_end]);

    // After a blank line, the remainder is instructions
    let _ = &input[crates_end + 2..].iter()
        .map(parse_instruction)
        .for_each(|inst| apply_instruction_v2(&mut crates, inst));

    let res = crates.iter()
        .map(|s| s[s.len() - 1]);
    return String::from_iter(res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_sample() {
        let sample_input = vec!["    [D]".to_string(), "[N] [C]    ".to_string(), "[Z] [M] [P]".to_string()];
        let actual = parse_stacks(&sample_input[..]);

        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];

        assert_eq!(actual, expected);
    }
}