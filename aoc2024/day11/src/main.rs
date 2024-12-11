fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    //println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .map(|num| blink(num, 0, 25))
        .sum()
}

fn blink(stone: usize, blink_n: usize, limit: usize) -> usize {
    if blink_n == limit {
        return 1;
    }

    if stone == 0 {
        return blink(1, blink_n + 1, limit);
    }

    let as_string = stone.to_string();
    if as_string.len() % 2 == 0 {
        let mid = (as_string.len() - 1) / 2;
        let stone1 = as_string[..=mid].parse::<usize>().unwrap();
        let stone2 = as_string[mid + 1..].parse::<usize>().unwrap();

        return blink(stone1, blink_n + 1, limit) + blink(stone2, blink_n + 1, limit);
    }

    return blink(stone * 2024, blink_n + 1, limit);
}
