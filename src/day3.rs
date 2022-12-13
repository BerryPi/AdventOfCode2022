use itertools::Itertools;

fn find_duplicated_item(items: &String) -> u8 {
    // String is guaranteed to be even-length
    let compartment_length = items.len() / 2;
    let (first, last) = items.split_at(compartment_length);

    // There is only one character in common between the halves
    let item = first.bytes().find_or_first(|c| last.bytes().contains(c));
    return item.unwrap();
}

fn item_score(item: u8) -> i32 {
    if item >= b'a' && item <= b'z' {
        return (item - b'`') as i32;
    }
    else if item >= b'A' && item <= b'Z' {
        return (item - b'@' + 26) as i32;
    }
    // This will not happen
    return 0;
}

pub fn part1(input: Vec<String>) -> i32 {
    return input.iter()
        .map(find_duplicated_item)
        .map(item_score)
        .sum();
}

fn find_badge(sacks: Vec<&String>) -> u8 {
    let first = sacks[0];
    let second = sacks[1];
    let third = sacks[2];

    let item = first.bytes().find_or_first(|c| second.bytes().contains(c) && third.bytes().contains(c));
    return item.unwrap();
}

pub fn part2(input: Vec<String>) -> i32 {
    return input.iter()
        .chunks(3)
        .into_iter()
        .map(|c| c.collect::<Vec<&String>>())
        .map(find_badge)
        .map(item_score)
        .sum();
}