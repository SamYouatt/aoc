use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

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

fn part_two(my_ticket: &[usize], nearby_tickets: &[Vec<usize>], rules: &[Rule]) -> usize {
    let valid_nearby_tickets: Vec<_> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|number| {
                rules.iter().any(|rule| {
                    (rule.first_range.start..=rule.first_range.end).contains(number)
                        || (rule.second_range.start..=rule.second_range.end).contains(number)
                })
            })
        })
        .collect();

    // maintain a hashmap of the instruction name to a hashset that tracks all possible indexes it could be valid
    // it does this by iterating through each rule, then iterating over indexes for ticket fields
    // it filters those for which every ticket with a value at that index is valid
    // for example with my input the field price has a hashset {3, 8, 18, 15, 7} where these values are possible
    // indexes that price could be. This is because for column 3, 8, etc. in the tickets, all the values are in the possible
    // range for price
    // for arrival track the hash set is just { 15 } this is important for the solving
    let mut names_left_to_map: HashMap<&str, HashSet<usize>> = rules
        .iter()
        .map(|rule| {
            (
                rule.name,
                (0..valid_nearby_tickets[0].len())
                    .filter(|i| {
                        valid_nearby_tickets
                            .iter()
                            .map(|ticket| ticket[*i])
                            .filter(|number| {
                                (rule.first_range.start..=rule.first_range.end).contains(number)
                                    || (rule.second_range.start..=rule.second_range.end)
                                        .contains(number)
                            })
                            // this is where it is checked that the field is applicable to every ticket
                            .count()
                            == valid_nearby_tickets.len()
                    })
                    .collect(),
            )
        })
        .collect();

    // create a vector that will be used to store the field names at the right index
    let mut field_names = vec![""; rules.len()];

    // for each rule find the first one for which there is only one number in its hash set
    // this means there is only one possible place that the rule could be
    // then set the index in the final vector of field names to that name
    // the field must then be removed from the hashmap of fields left to map, as well as the index
    // being removed from all the hash sets
    // this should mean that the next time round there is another hashset that now only has one value
    (0..field_names.len()).for_each(|_| {
        let (&name, appearances) = names_left_to_map
            .iter()
            .find(|(_, appearances)| appearances.len() == 1)
            .unwrap();

        let position = *appearances.iter().next().unwrap();
        field_names[position] = name;

        names_left_to_map.values_mut().for_each(|set| {
            set.remove(&position);
        });

        names_left_to_map.remove(name);
    });

    my_ticket
        .iter()
        .enumerate()
        .map(|(i, value)| (field_names[i], value))
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, value)| value)
        .product()
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
