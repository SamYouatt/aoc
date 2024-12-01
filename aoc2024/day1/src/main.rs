fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(&input));
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
