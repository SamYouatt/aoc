fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let regex = regex::Regex::new(r"mul\((\d+,\d+)\)").expect("invalid regex dummy");

    regex
        .captures_iter(input)
        .map(|c| c.extract::<1>().0)
        .map(|content| {
            let (_, rem) = content.split_once('(').unwrap();
            let (rem, _) = rem.split_once(')').unwrap();
            let (x, y) = rem.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .fold(0, |acc, (x, y)| acc + (x * y))
}

fn part_2(input: &str) -> usize {
    let regex =
        regex::Regex::new(r"mul\((\d+,\d+)\)|(do\(\))|(don't\(\))").expect("invalid regex dummy");

    let mut total = 0;
    let mut enabled = true;

    for capture in regex.captures_iter(input).map(|c| c.extract::<1>().0) {
        if capture.starts_with("do(") {
            enabled = true;
            continue;
        }
        if capture.starts_with("don't") {
            enabled = false;
            continue;
        }

        if enabled {
            let (_, rem) = capture.split_once('(').unwrap();
            let (rem, _) = rem.split_once(')').unwrap();
            let (x, y) = rem.split_once(',').unwrap();
            let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());

            total += x * y;
        }
    }

    total
}
