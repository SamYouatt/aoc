use std::time::Instant;

use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/016.txt");

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let reg_rule = Regex::new(r#"^*: (\d+)-(\d+) or (\d+)-(\d+)$"#).unwrap();

    let rules: Vec<Rule> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|rule| {
            let captured = reg_rule.captures(rule).unwrap();

            Rule {
                first_range: Range::new(
                    captured[1].parse::<usize>().unwrap(),
                    captured[2].parse::<usize>().unwrap(),
                ),
                second_range: Range::new(
                    captured[3].parse::<usize>().unwrap(),
                    captured[4].parse::<usize>().unwrap(),
                ),
            }
        })
        .collect();

    input
        .lines()
        .skip(rules.len() + 5)
        .flat_map(|line| {
            line.split(',')
                .map(|number| number.parse::<usize>().unwrap())
        })
        .filter(|number| {
            !rules.iter().any(|rule| {
                (rule.first_range.start..=rule.first_range.end).contains(number)
                    || (rule.second_range.start..=rule.second_range.end).contains(number)
            })
        })
        .sum::<usize>()
}

#[derive(Debug)]
struct Rule {
    first_range: Range,
    second_range: Range,
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range { start, end }
    }
}
