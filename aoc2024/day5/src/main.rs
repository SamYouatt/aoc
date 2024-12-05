use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules: Vec<(usize, usize)> = rules
        .lines()
        .map(|line| line.split_once('|').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let pages: Vec<Vec<_>> = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    pages
        .into_iter()
        .filter(|pages| is_valid_pages(pages, &rules))
        .map(|pages| pages[(pages.len() - 1) / 2])
        .sum()
}

fn is_valid_pages(pages: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    for (&x, &y) in pages.iter().tuple_windows() {
        for &rule in rules {
            if (x != rule.0 && x != rule.1) || (y != rule.0 && y != rule.1) {
                continue;
            }

            // I think its fine to just test the direct pairs and not think further out?
            if x == rule.1 && y == rule.0 {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        assert_eq!(part_1(input), 143);
    }
}
