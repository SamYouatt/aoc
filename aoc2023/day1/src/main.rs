fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(&input);
    println!("part 1: {}", answer1);
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let digits: Vec<char> = line.chars().filter(|char| char.is_digit(10)).collect();
            let first_digit = digits.first().unwrap_or(&'0');
            let last_digit = digits.last().unwrap_or(&'0');

            format!("{}{}", first_digit, last_digit)
                .parse::<usize>()
                .unwrap_or(0)
        })
        .sum()
}
