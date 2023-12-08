use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
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

fn part_2(input: &str) -> usize {
    let (instructions, rest) = input.split_once("\n\n").unwrap();
    let instructions = instructions.chars().collect::<Vec<_>>();

    let nodes: HashMap<_, (_, _)> = rest
        .lines()
        .map(|line| {
            let line_bytes = line.as_bytes();

            (&line_bytes[0..3], (&line_bytes[7..10], &line_bytes[12..15]))
        })
        .collect();

    let steps_for_starts: Vec<_> = nodes
        .keys()
        .filter(|key| key.ends_with(b"A"))
        .map(|node| calculate_steps(&instructions, &nodes, node))
        .collect();

    lcm(&steps_for_starts)
}

fn calculate_steps(
    instructions: &Vec<char>,
    nodes: &HashMap<&[u8], (&[u8], &[u8])>,
    start: &[u8],
) -> usize {
    let mut current = start;
    let mut steps = 0;

    while !current.ends_with(b"Z") {
        for instruction in instructions {
            current = if instruction == &'L' {
                nodes[current].0
            } else {
                nodes[current].1
            };
            steps += 1
        }
    }

    steps
}

fn lcm(numbers: &[usize]) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }

    let a = numbers[0];
    let b = lcm(&numbers[1..]);

    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
