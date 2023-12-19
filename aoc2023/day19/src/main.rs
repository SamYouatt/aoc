use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
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
