use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let (mut pages, page_to_preds) = parse(input);

    println!("Part 1: {}", part_1(&pages, &page_to_preds));
    println!("Part 2: {}", part_2(&mut pages, &page_to_preds));
}

fn part_1(pages: &Vec<Vec<usize>>, page_to_pred: &HashMap<usize, Vec<usize>>) -> usize {
    pages
        .iter()
        .filter(|pages| pages.is_sorted_by(|a, b| page_to_pred[b].contains(a)))
        .map(|pages| pages[(pages.len() - 1) / 2])
        .sum()
}

fn part_2(all_pages: &mut Vec<Vec<usize>>, page_to_pred: &HashMap<usize, Vec<usize>>) -> usize {
    let bad_pages = all_pages
        .into_iter()
        .filter(|pages| !pages.is_sorted_by(|a, b| page_to_pred[b].contains(a)))
        .collect::<Vec<_>>();

    let mut total = 0;
    for pages in bad_pages {
        pages.sort_by(|a, b| page_to_pred[b].contains(a).cmp(&true));
        total += pages[(pages.len() - 1) / 2];
    }

    return total;
}

fn parse(input: &str) -> (Vec<Vec<usize>>, HashMap<usize, Vec<usize>>) {
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

    let mut page_to_preds = HashMap::new();
    for rule in rules.iter() {
        page_to_preds.entry(rule.1).or_insert(vec![]).push(rule.0);
    }

    (pages, page_to_preds)
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

        let (pages, map) = parse(input);
        assert_eq!(part_1(&pages, &map), 143);
    }

    #[test]
    fn test_part_2() {
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

        let (mut pages, map) = parse(input);
        assert_eq!(part_2(&mut pages, &map), 123);
    }
}
