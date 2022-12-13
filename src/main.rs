use std::io::{self, BufRead};
use std::fs::File;
mod day1;
mod day2;
mod day3;

fn main() {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut line).expect("Could not read the problem code.");

    if line.starts_with("1-") {
        let file = File::open("./inputs/day1.txt").unwrap();
        let buf_reader = io::BufReader::new(file).lines();
        let lines = buf_reader.map(|l| l.unwrap()).collect::<Vec<String>>();

        if line.starts_with("1-1") {
            println!("{}", day1::part1(lines));
        }
        else if line.starts_with("1-2") {
            println!("{}", day1::part2(lines));
        }
    }

    else if line.starts_with("2-") {
        let file = File::open("./inputs/day2.txt").unwrap();
        let buf_reader = io::BufReader::new(file).lines();
        let lines = buf_reader.map(|l| l.unwrap()).collect::<Vec<String>>();

        if line.starts_with("2-1") {
            println!("{}", day2::part1(lines));
        }
        else if line.starts_with("2-2") {
            println!("{}", day2::part2(lines));
        }
    }

    else if line.starts_with("3-") {
        let file = File::open("./inputs/day3.txt").unwrap();
        let buf_reader = io::BufReader::new(file).lines();
        let lines = buf_reader.map(|l| l.unwrap()).collect::<Vec<String>>();

        if line.starts_with("3-1") {
            println!("{}", day3::part1(lines));
        }
        else if line.starts_with("3-2") {
            println!("{}", day3::part2(lines));
        }
    }
}