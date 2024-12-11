fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut stones = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        stones = blink(stones);
    }

    stones.len()
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    let mut new = Vec::new();

    for stone in stones {
        if stone == 0 {
            new.push(1);
            continue;
        }

        let as_string = stone.to_string();
        if as_string.len() % 2 == 0 {
            let mid = (as_string.len() - 1) / 2;
            let stone1 = as_string[..=mid].parse::<usize>().unwrap();
            let stone2 = as_string[mid + 1..].parse::<usize>().unwrap();

            new.push(stone1);
            new.push(stone2);
            continue;
        }

        new.push(stone * 2024);
    }

    new
}
