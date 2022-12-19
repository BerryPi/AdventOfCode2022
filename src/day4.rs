fn parse_range(range: &str) -> (i32, i32) {
    let (start, end) = range.split_once('-').unwrap();
    (start.parse::<i32>().unwrap(), end.parse::<i32>().unwrap())
}

fn parse_ranges(ranges: &str) -> ((i32, i32), (i32, i32)) {
    let (range1, range2) = ranges.split_once(',').unwrap();
    (parse_range(range1), parse_range(range2))
}

fn do_ranges_contain(range1: &(i32, i32), range2: &(i32, i32)) -> bool {
    // A range contains the other if its start is at or before the other's start,
    // and its end is at or after the other's end
    if range1.0 >= range2.0 && range1.1 <= range2.1 {
        return true;
    }
    range1.0 <= range2.0 && range1.1 >= range2.1
}

fn do_ranges_overlap(range1: &(i32, i32), range2: &(i32, i32)) -> bool {
    // Ranges overlap if the start of one is between the start and end of the other
    if range1.0 >= range2.0 && range1.0 <= range2.1 {
        return true;
    }
    range2.0 >= range1.0 && range2.0 <= range1.1
}

pub fn part1(input: Vec<String>) -> String {
    let containments = input.iter()
        .map(|s| parse_ranges(s))
        .filter(|(range1, range2)| do_ranges_contain(range1, range2));
    
    containments.count().to_string()
}

pub fn part2(input: Vec<String>) -> String {
    let containments = input.iter()
        .map(|s| parse_ranges(s))
        .filter(|(range1, range2)| do_ranges_overlap(range1, range2));
    
    containments.count().to_string()
}