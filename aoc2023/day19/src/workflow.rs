use crate::rule::Goto;
use crate::rule::Rule;
use crate::stat::Stat;
use crate::Part;

#[derive(Debug)]
pub enum Action {
    Accepted,
    Goto(String),
}

#[derive(Debug)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
}

impl Workflow {
    pub fn parse(line: &str) -> Workflow {
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

    pub fn act(&self, part: &Part) -> Option<Action> {
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
