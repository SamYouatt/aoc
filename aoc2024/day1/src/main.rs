use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (mut list_x, mut list_y): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .unzip();

    list_x.sort();
    list_y.sort();

    list_x
        .iter()
        .zip(list_y.iter())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

fn part2(input: &str) -> usize {
    let (list_x, list_y): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
        .unzip();

    let list_y_freq = list_y.into_iter().fold(HashMap::new(), |mut acc, y| {
        *acc.entry(y).or_insert(0) += 1;
        acc
    });

    list_x
        .iter()
        .map(|x| x * list_y_freq.get(x).unwrap_or(&0))
        .sum()
}
