use std::io::{self, BufRead};
use std::fs::File;
mod day1;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Could not read the problem code.");

    if line.starts_with("1-") {
        let file = File::open("./inputs/day1.txt").unwrap();
        let lines = io::BufReader::new(file).lines();

        if line.starts_with("1-1") {
            println!("{}", day1::part1(lines));
        }
        else if line.starts_with("1-2") {
            println!("{}", day1::part2(lines));
        }
    }
}