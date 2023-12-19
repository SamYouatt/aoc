use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(line: &str) -> Part {
        let cleaned = line.replace("{", "").replace("}", "");
        let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);

        cleaned.split(",").for_each(|score| {
            let (stat, rating) = score.split_once("=").unwrap();

            match stat {
                "x" => x = rating.parse::<usize>().unwrap(),
                "m" => m = rating.parse::<usize>().unwrap(),
                "a" => a = rating.parse::<usize>().unwrap(),
                "s" => s = rating.parse::<usize>().unwrap(),
                _ => panic!("Unknown stat"),
            };
        });

        Part { x, m, a, s }
    }

    fn get_stat(&self, stat: &Stat) -> usize {
        match stat {
            Stat::XCool => self.x,
            Stat::Musical => self.m,
            Stat::Aero => self.a,
            Stat::Shiny => self.s,
        }
    }
}

#[derive(Debug)]
enum Stat {
    XCool,
    Musical,
    Aero,
    Shiny,
}

impl Stat {
    fn parse(raw: &str) -> Stat {
        match raw {
            "x" => Stat::XCool,
            "m" => Stat::Musical,
            "a" => Stat::Aero,
            "s" => Stat::Shiny,
            _ => panic!("Unknown stat"),
        }
    }
}

#[derive(Debug)]
enum Goto {
    Loc(String),
    Accepted,
    Rejected,
}

impl Goto {
    fn parse(loc: &str) -> Goto {
        match loc {
            "A" => Goto::Accepted,
            "R" => Goto::Rejected,
            x => Goto::Loc(String::from(x)),
        }
    }

    fn raw_location(&self) -> String {
        match self {
            Goto::Loc(loc) => String::from(loc),
            Goto::Accepted => String::from("A"),
            Goto::Rejected => String::from("R"),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Accepted,
    Rejected,
    Goto(String),
    Greater(Stat, usize, Goto),
    Less(Stat, usize, Goto),
}

#[derive(Debug)]
enum Action {
    Accepted,
    Goto(String),
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(line: &str) -> Workflow {
        let (name, rest) = line.split_once("{").unwrap();
        let raw_rules = rest.replace("}", "");

        let rules: Vec<_> = raw_rules
            .split(",")
            .map(|r| match r {
                r if r.contains(">") => {
                    let (stat, rest) = r.split_once(">").unwrap();
                    let (goal, goto) = rest.split_once(":").unwrap();

                    Rule::Greater(
                        Stat::parse(stat),
                        goal.parse::<usize>().unwrap(),
                        Goto::parse(goto),
                    )
                }
                r if r.contains("<") => {
                    let (stat, rest) = r.split_once("<").unwrap();
                    let (goal, goto) = rest.split_once(":").unwrap();

                    Rule::Less(
                        Stat::parse(stat),
                        goal.parse::<usize>().unwrap(),
                        Goto::parse(goto),
                    )
                }
                "A" => Rule::Accepted,
                "R" => Rule::Rejected,
                _ => Rule::Goto(String::from(r)),
            })
            .collect();

        Workflow {
            name: String::from(name),
            rules,
        }
    }

    fn act(&self, part: &Part) -> Option<Action> {
        for rule in &self.rules {
            match rule {
                Rule::Accepted => return Some(Action::Accepted),
                Rule::Rejected => return None,
                Rule::Goto(x) => return Some(Action::Goto(String::from(x))),
                Rule::Greater(stat, goal, goto) => {
                    let part_stat = part.get_stat(stat);

                    if &part_stat > goal {
                        return match goto {
                            Goto::Loc(x) => Some(Action::Goto(String::from(x))),
                            Goto::Accepted => Some(Action::Accepted),
                            Goto::Rejected => None,
                        };
                    } else {
                        continue;
                    }
                }
                Rule::Less(stat, goal, goto) => {
                    let part_stat = part.get_stat(stat);

                    if &part_stat < goal {
                        return match goto {
                            Goto::Loc(x) => Some(Action::Goto(String::from(x))),
                            Goto::Accepted => Some(Action::Accepted),
                            Goto::Rejected => None,
                        };
                    } else {
                        continue;
                    }
                }
            }
        }

        None
    }
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

#[derive(Clone, Copy)]
struct StatRange {
    start: usize,
    end: usize,
}

impl StatRange {
    fn new(start: usize, end: usize) -> StatRange {
        StatRange { start, end }
    }

    fn size(&self) -> usize {
        self.end - self.start + 1
    }
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
