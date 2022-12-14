use std::io::{self, BufRead};
use std::fs::File;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Could not read the problem code.");
    
    let problem = line.trim();

    let (problem_num, _part_num) = line.split_once('-').expect("Invalid problem number.");
    let input_file = format!("./inputs/day{}.txt", problem_num);

    let file = File::open(input_file).unwrap();
    let buf_reader = io::BufReader::new(file).lines();
    let lines = buf_reader.map(|l| l.unwrap()).collect::<Vec<String>>();

    let problem_fn = match problem {
        "1-1" => day1::part1,
        "1-2" => day1::part2,
        "2-1" => day2::part1,
        "2-2" => day2::part2,
        "3-1" => day3::part1,
        "3-2" => day3::part2,
        "4-1" => day4::part1,
        "4-2" => day4::part2,
        "5-1" => day5::part1,
        "5-2" => day5::part2,
        _ => panic!("Problem is invalid or not yet implemented.")
    };

    println!("{}", problem_fn(lines));
}