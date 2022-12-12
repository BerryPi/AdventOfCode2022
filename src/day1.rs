use itertools::Itertools;

fn sorted_elf_totals(input: Vec<String>) -> impl Iterator<Item = i32> {
    let elf_items = input.split(|line| line == "");
    let elf_numbers = elf_items.map(|items| items.iter().map(|s| s.parse::<i32>().unwrap()));
    let elf_totals = elf_numbers.map(|group| group.sum::<i32>());

    return elf_totals.sorted_by(|a, b| b.cmp(a));
}

pub fn part1(input: Vec<String>) -> i32 {
    let mut elf_totals = sorted_elf_totals(input);

    let max = elf_totals.next().unwrap();
    return max;
}

pub fn part2(input: Vec<String>) -> i32 {
    let elf_totals = sorted_elf_totals(input);

    let top_n = elf_totals.take(3);
    let total = top_n.sum();
    return total;
}