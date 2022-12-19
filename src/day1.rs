use itertools::Itertools;

fn sorted_elf_totals(input: Vec<String>) -> impl Iterator<Item = i32> {
    let elf_items = input.split(|line| line.is_empty());
    let elf_numbers = elf_items.map(|items| items.iter().map(|s| s.parse::<i32>().unwrap()));
    let elf_totals = elf_numbers.map(|group| group.sum::<i32>());

    elf_totals.sorted_by(|a, b| b.cmp(a))
}

pub fn part1(input: Vec<String>) -> String {
    let mut elf_totals = sorted_elf_totals(input);

    let max = elf_totals.next().unwrap();

    max.to_string()
}

pub fn part2(input: Vec<String>) -> String {
    let elf_totals = sorted_elf_totals(input);

    let top_n = elf_totals.take(3);
    let total: i32 = top_n.sum();

    total.to_string()
}