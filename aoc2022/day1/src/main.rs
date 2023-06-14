fn main() {
    let input = include_str!("input.txt");

    let answer1: usize = input
        .split("\n\n")
        .map(|chunk| (chunk.lines().map(|line| line.parse::<usize>().unwrap())).sum())
        .max()
        .unwrap();

    println!("part 1: {}", answer1);

    let mut elf_sums = input
        .split("\n\n")
        .map(|chunk| (chunk.lines().map(|line| line.parse::<usize>().unwrap())).sum::<usize>())
        .collect::<Vec<usize>>();

    elf_sums.sort_unstable();

    let answer2: usize = elf_sums.into_iter().rev().take(3).sum();

    println!("part 2: {}", answer2);
}
