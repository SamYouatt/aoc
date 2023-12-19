use crate::stat::Stat;

#[derive(Debug)]
pub enum Goto {
    Loc(String),
    Accepted,
    Rejected,
}

impl Goto {
    pub fn parse(loc: &str) -> Goto {
        match loc {
            "A" => Goto::Accepted,
            "R" => Goto::Rejected,
            x => Goto::Loc(String::from(x)),
        }
    }

    pub fn raw_location(&self) -> String {
        match self {
            Goto::Loc(loc) => String::from(loc),
            Goto::Accepted => String::from("A"),
            Goto::Rejected => String::from("R"),
        }
    }
}

#[derive(Debug)]
pub enum Rule {
    Accepted,
    Rejected,
    Goto(String),
    Greater(Stat, usize, Goto),
    Less(Stat, usize, Goto),
}
