use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .fold(0, |acc, num| acc + blink(num, 0, 25, &mut HashMap::new()))
}

fn part_2(input: &str) -> usize {
    input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .fold(0, |acc, num| acc + blink(num, 0, 75, &mut HashMap::new()))
}

fn blink(
    stone: usize,
    blink_n: usize,
    limit: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blink_n == limit {
        return 1;
    }

    if let Some(cached) = cache.get(&(stone, blink_n)) {
        return *cached;
    }

    let result = match stone {
        0 => blink(1, blink_n + 1, limit, cache),
        x if x.to_string().len() % 2 == 0 => {
            let as_string = stone.to_string();
            let mid = (as_string.len() - 1) / 2;
            let stone1 = as_string[..=mid].parse::<usize>().unwrap();
            let stone2 = as_string[mid + 1..].parse::<usize>().unwrap();

            blink(stone1, blink_n + 1, limit, cache) + blink(stone2, blink_n + 1, limit, cache)
        }
        _ => blink(stone * 2024, blink_n + 1, limit, cache),
    };

    cache.insert((stone, blink_n), result);
    result
}
