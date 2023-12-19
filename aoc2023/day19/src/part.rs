use crate::stat::Stat;

#[derive(Debug, Clone, Copy)]
pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl Part {
    pub fn parse(line: &str) -> Part {
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

    pub fn get_stat(&self, stat: &Stat) -> usize {
        match stat {
            Stat::XCool => self.x,
            Stat::Musical => self.m,
            Stat::Aero => self.a,
            Stat::Shiny => self.s,
        }
    }
}
