use regex::Regex;

#[derive(Copy, Clone)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn from_parse(set: &str) -> Set {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let colours = set.trim().split(", ");

        for colour in colours {
            let pieces = colour.split(" ").collect::<Vec<&str>>();

            let count = pieces[0]
                .parse::<u32>()
                .expect("failed to parse colour number");

            match pieces[1] {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => {}
            }
        }

        Set { red, green, blue }
    }

    fn is_valid(&self, red_limit: u32, green_limit: u32, blue_limit: u32) -> bool {
        self.red <= red_limit && self.green <= green_limit && self.blue <= blue_limit
    }
}

struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn from_parse(game: &str) -> Game {
        let capture_id = Regex::new(r"Game (\d+):").unwrap();

        let id = match capture_id.captures(game) {
            Some(captures) => match captures.get(1) {
                Some(id) => id.as_str().parse::<u32>().expect("failed to parse game id"),
                _ => 0,
            },
            _ => 0,
        };

        let sets: Vec<Set> = game[game.find(':').unwrap() + 2..]
            .split(';')
            .map(|set| Set::from_parse(set))
            .collect();

        Game { id, sets }
    }

    fn get_min_set(&self) -> Set {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in &self.sets {
            max_red = max_red.max(set.red);
            max_green = max_green.max(set.green);
            max_blue = max_blue.max(set.blue);
        }

        Set {
            red: max_red,
            green: max_green,
            blue: max_blue,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {}", answer1);

    let answer2 = part_2(input);
    println!("Part 2: {}", answer2);
}

fn part_1(input: &str) -> u32 {
    let (red_limit, green_limit, blue_limit) = (12, 13, 14);

    input
        .lines()
        .map(|line| Game::from_parse(line))
        .filter(|game| {
            game.sets
                .iter()
                .all(|set| set.is_valid(red_limit, green_limit, blue_limit))
        })
        .fold(0, |total, game| total + game.id)
}

fn part_2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| Game::from_parse(line))
        .map(|game| game.get_min_set())
        .fold(0, |total, min_set| {
            total + (min_set.red * min_set.green * min_set.blue)
        })
}
