use itertools::Itertools;

fn all_distinct(window: &str) -> bool {
    return window.chars().duplicates().count() == 0;
}

fn find_distinct_window(input: Vec<String>, window_size: usize) -> String {
    // There is only one line
    let line = &input[0];
    for i in 0..(line.len() - window_size) {
        if all_distinct(&line[i..i+window_size]) {
            // We need the count of *processed* characters,
            // which is one past the index of the last character
            // in the slice
            return (i + window_size).to_string();
        }
    }
    return "".to_string();
}

pub fn part1(input: Vec<String>) -> String {
    return find_distinct_window(input, 4);
}

pub fn part2(input: Vec<String>) -> String {
    return find_distinct_window(input, 14);
}