fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let regex = regex::Regex::new(r#"mul\((\d+,\d+)\)"#).expect("invalid regex dummy");

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
