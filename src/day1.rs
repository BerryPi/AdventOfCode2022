use std::{io::{Lines, BufReader}, fs::File};

use itertools::Itertools;

fn sorted_elf_totals(input: Lines<BufReader<File>>) -> impl Iterator<Item = i32> {
    let iter = input.map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let elf_items = iter.split(|line| line == "");
    let elf_numbers = elf_items.map(|items| items.iter().map(|s| s.parse::<i32>().unwrap()));
    let elf_totals = elf_numbers.map(|group| group.sum::<i32>());

    return elf_totals.sorted_by(|a, b| b.cmp(a));
}

pub fn part1(input: Lines<BufReader<File>>) -> i32 {
    let mut elf_totals = sorted_elf_totals(input);

    let max = elf_totals.next().unwrap();
    return max;
}

pub fn part2(input: Lines<BufReader<File>>) -> i32 {
    let elf_totals = sorted_elf_totals(input);

    let top_n = elf_totals.take(3);
    let total = top_n.sum();
    return total;
}