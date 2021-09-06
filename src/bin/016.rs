use std::time::Instant;

use regex::Regex;

fn main() {
    let input = include_str!("../../inputs/016.txt");

    let rules: Vec<Rule> = read_rules(input);

    let my_ticket: Vec<usize> = read_tickets("your ticket:", input).next().unwrap();

    let nearby_tickets: Vec<_> = read_tickets("nearby tickets:", input).collect();

    let start = Instant::now();
    println!(
        "Part one: {} in {:#?}",
        part_one(&nearby_tickets, &rules),
        start.elapsed()
    );

    let start = Instant::now();
    println!(
        "Part two: {} in {:#?}",
        part_two(&my_ticket, &nearby_tickets, &rules),
        start.elapsed()
    );
}

fn part_one(tickets: &[Vec<usize>], rules: &[Rule]) -> usize {
    tickets
        .iter()
        .flatten()
        .filter(|number| {
            !rules.iter().any(|rule| {
                (rule.first_range.start..=rule.first_range.end).contains(number)
                    || (rule.second_range.start..=rule.second_range.end).contains(number)
            })
        })
        .sum::<usize>()
}

fn part_two(my_ticket: &[usize], tickets: &[Vec<usize>], rules: &[Rule]) -> usize {
    let valid_nearby_tickets = tickets.iter().filter(|ticket| {
        ticket.iter().all(|number| {
            rules.iter().any(|rule| {
                (rule.first_range.start..=rule.first_range.end).contains(number)
                    || (rule.second_range.start..=rule.second_range.end).contains(number)
            })
        })
    });

    0
}

fn read_rules(input: &str) -> Vec<Rule> {
    let reg_rule = Regex::new(r#"^(.*): (\d+)-(\d+) or (\d+)-(\d+)$"#).unwrap();

    input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|rule| {
            let captured = reg_rule.captures(rule).unwrap();

            Rule {
                name: captured.get(1).unwrap().as_str(),
                first_range: Range::new(
                    captured[2].parse::<usize>().unwrap(),
                    captured[3].parse::<usize>().unwrap(),
                ),
                second_range: Range::new(
                    captured[4].parse::<usize>().unwrap(),
                    captured[5].parse::<usize>().unwrap(),
                ),
            }
        })
        .collect()
}

// header and input must both have lifetime of static because the returned iterator could live forever (static)
fn read_tickets(header: &'static str, input: &'static str) -> impl Iterator<Item = Vec<usize>> {
    input
        .lines()
        .skip_while(move |&line| line != header)
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|numbers| {
            numbers
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect()
        })
}

#[derive(Debug)]
struct Rule<'a> {
    name: &'a str,
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

struct Ticket {
    values: Vec<usize>,
}
