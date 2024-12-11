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
        _ => {
            let digits = stone.ilog10() + 1;
            if digits % 2 == 0 {
                let stone1 = stone as isize / 10_isize.pow(digits / 2);
                let stone2 = stone as isize % 10_isize.pow(digits / 2);
                blink(stone1 as usize, blink_n + 1, limit, cache)
                    + blink(stone2 as usize, blink_n + 1, limit, cache)
            } else {
                blink(stone * 2024, blink_n + 1, limit, cache)
            }
        }
    };

    cache.insert((stone, blink_n), result);
    result
}
