fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<usize>().expect("input is only numbers"))
                .collect::<Vec<_>>()
        })
        .filter(|r| is_safe(r))
        .count()
}

fn is_safe(report: &[usize]) -> bool {
    let ascending = report[0] < report[1];

    for i in 0..report.len() - 1 {
        match (report[i], report[i + 1], ascending) {
            (x, y, true) if x > y => return false,
            (x, y, false) if x < y => return false,
            (x, y, _) if x.abs_diff(y) < 1 || x.abs_diff(y) > 3 => return false,
            _ => {}
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = r#" 7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9 "#;

        assert_eq!(part_1(input), 2);
    }
}
