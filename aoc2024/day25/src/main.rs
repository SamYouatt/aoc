fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for chunk in input.split("\n\n") {
        if chunk.trim().starts_with("#####") {
            locks.push(parse(chunk, true));
        } else {
            keys.push(parse(chunk, false));
        }
    }

    let mut total = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if fits(&key, &lock) {
                total += 1;
            }
        }
    }

    total
}

fn parse(input: &str, lock: bool) -> Vec<usize> {
    let mut heights = vec![0; 5];

    for (i, line) in input.lines().enumerate() {
        if (lock && i == 0) || (!lock && i == 6) {
            continue;
        }

        for (i, char) in line.trim().bytes().enumerate() {
            if char == b'#' {
                heights[i] += 1;
            }
        }
    }

    heights
}

fn fits(key: &Vec<usize>, lock: &Vec<usize>) -> bool {
    key.iter().enumerate().all(|(i, x)| lock[i] + x <= 5)
}
