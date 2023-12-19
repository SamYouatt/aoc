mod part;
mod rule;
mod stat;
mod stat_range;
mod workflow;

use crate::part::Part;
use crate::rule::Rule;
use crate::stat::Stat;
use crate::stat_range::StatRange;
use crate::workflow::Action;
use crate::workflow::Workflow;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

fn part_1(input: &str) -> usize {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| Workflow::parse(line))
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();
    let parts: Vec<_> = parts.lines().map(|line| Part::parse(line)).collect();

    let mut accepted: Vec<Part> = vec![];

    for part in parts {
        let mut current_flow = workflows.get("in").unwrap();

        while let Some(action) = current_flow.act(&part) {
            match action {
                Action::Accepted => {
                    accepted.push(part.clone());
                    break;
                }
                Action::Goto(x) => current_flow = workflows.get(&x).unwrap(),
            }
        }
    }

    accepted
        .iter()
        .fold(0, |total, part| total + part.x + part.m + part.s + part.a)
}

fn part_2(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").unwrap();

    let workflows: HashMap<String, Workflow> = workflows
        .lines()
        .map(|line| Workflow::parse(line))
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect();

    let mut rule_stack: Vec<(StatRange, StatRange, StatRange, StatRange, usize, String)> = vec![(
        StatRange::new(1, 4000),
        StatRange::new(1, 4000),
        StatRange::new(1, 4000),
        StatRange::new(1, 4000),
        0,
        String::from("in"),
    )];

    let mut ranges: Vec<(StatRange, StatRange, StatRange, StatRange)> = vec![];

    while let Some(next) = rule_stack.pop() {
        let (x, m, a, s, rule_index, workflow_name) = next;

        if workflow_name == "R" {
            continue;
        }

        if workflow_name == "A" {
            ranges.push((x, m, a, s));
            continue;
        }

        let workflow = workflows.get(&workflow_name).unwrap();

        match &workflow.rules[rule_index] {
            Rule::Accepted => {
                ranges.push((x, m, a, s));
                continue;
            }
            Rule::Rejected => continue,
            Rule::Goto(next_workflow_name) => {
                rule_stack.push((x, m, a, s, 0, next_workflow_name.clone()));
                continue;
            }
            Rule::Greater(stat, goal, goto) => match stat {
                Stat::XCool => {
                    let accepted_range = StatRange::new(goal + 1, x.end);
                    rule_stack.push((accepted_range, m, a, s, 0, goto.raw_location()));

                    let rejected_range = StatRange::new(x.start, *goal);
                    rule_stack.push((rejected_range, m, a, s, rule_index + 1, workflow_name));
                }
                Stat::Musical => {
                    let accepted_range = StatRange::new(goal + 1, m.end);
                    rule_stack.push((x, accepted_range, a, s, 0, goto.raw_location()));

                    let rejected_range = StatRange::new(m.start, *goal);
                    rule_stack.push((x, rejected_range, a, s, rule_index + 1, workflow_name));
                }
                Stat::Aero => {
                    let accepted_range = StatRange::new(goal + 1, a.end);
                    rule_stack.push((x, m, accepted_range, s, 0, goto.raw_location()));

                    let rejected_range = StatRange::new(a.start, *goal);
                    rule_stack.push((x, m, rejected_range, s, rule_index + 1, workflow_name));
                }
                Stat::Shiny => {
                    let accepted_range = StatRange::new(goal + 1, s.end);
                    rule_stack.push((x, m, a, accepted_range, 0, goto.raw_location()));

                    let rejected_range = StatRange::new(s.start, *goal);
                    rule_stack.push((x, m, a, rejected_range, rule_index + 1, workflow_name));
                }
            },
            Rule::Less(stat, goal, goto) => match stat {
                Stat::XCool => {
                    let accepted_range = StatRange::new(x.start, goal - 1);
                    rule_stack.push((accepted_range, m, a, s, 0, goto.raw_location()));

                    let rejected_range = StatRange::new(*goal, x.end);
                    rule_stack.push((rejected_range, m, a, s, rule_index + 1, workflow_name));
                }
                Stat::Musical => {
                    let accepted_range = StatRange::new(m.start, goal - 1);
                    rule_stack.push((x, accepted_range, a, s, 0, goto.raw_location()));

                    let rejected_range = StatRange::new(*goal, m.end);
                    rule_stack.push((x, rejected_range, a, s, rule_index + 1, workflow_name));
                }
                Stat::Aero => {
                    let accepted_range = StatRange::new(a.start, goal - 1);
                    rule_stack.push((x, m, accepted_range, s, 0, goto.raw_location()));

                    let rejected_range = StatRange::new(*goal, a.end);
                    rule_stack.push((x, m, rejected_range, s, rule_index + 1, workflow_name));
                }
                Stat::Shiny => {
                    let accepted_range = StatRange::new(s.start, goal - 1);
                    rule_stack.push((x, m, a, accepted_range, 0, goto.raw_location()));

                    let rejected_range = StatRange::new(*goal, s.end);
                    rule_stack.push((x, m, a, rejected_range, rule_index + 1, workflow_name));
                }
            },
        }
    }

    ranges.iter().fold(0, |total, (x, m, a, s)| {
        total + x.size() * m.size() * a.size() * s.size()
    })
}
