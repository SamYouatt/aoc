use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

fn part_1(input: &str) -> usize {
    let (instructions, rest) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().collect::<Vec<_>>();

    let nodes: HashMap<_, (_, _)> = rest
        .lines()
        .map(|line| {
            let line_bytes = line.as_bytes();

            (&line_bytes[0..3], (&line_bytes[7..10], &line_bytes[12..15]))
        })
        .collect();

    let mut steps = 0;
    let mut current: &[u8] = b"AAA";

    while current != b"ZZZ" {
        for instruction in &instructions {
            current = if instruction == &'L' {
                nodes[current].0
            } else {
                nodes[current].1
            };
            steps += 1;
        }
    }

    steps
}
