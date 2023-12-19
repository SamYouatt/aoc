#[derive(Debug)]
pub enum Stat {
    XCool,
    Musical,
    Aero,
    Shiny,
}

impl Stat {
    pub fn parse(raw: &str) -> Stat {
        match raw {
            "x" => Stat::XCool,
            "m" => Stat::Musical,
            "a" => Stat::Aero,
            "s" => Stat::Shiny,
            _ => panic!("Unknown stat"),
        }
    }
}
