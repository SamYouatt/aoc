use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Number {
    occupies: Vec<Location>,
    number: usize,
}

impl Number {
    fn new(start: usize, end: usize, row: usize, number: usize) -> Number {
        let start_i = start as i32;
        let end_i = end as i32;
        let row_i = row as i32;

        Number {
            occupies: (start_i..=end_i)
                .map(|x| Location { x, y: row_i })
                .collect(),
            number,
        }
    }

    fn is_adjacent(&self, symbols: &HashSet<Location>) -> bool {
        self.occupies.iter().any(|loc| {
            (-1..=1)
                .flat_map(|x_off| (-1..=1).map(move |y_off| (x_off, y_off)))
                .any(|(x_off, y_off)| {
                    symbols.contains(&Location {
                        x: loc.x + x_off,
                        y: loc.y + y_off,
                    })
                })
        })
    }

    fn is_at_location(&self, location: &Location) -> bool {
        self.occupies.iter().any(|loc| loc == location)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {}", answer1);

    let answer2 = part_2(input);
    println!("Part 2: {}", answer2);
}

fn part_1(input: &str) -> usize {
    let match_symbol = Regex::new(r"[^\d.]").expect("Failed to compile regex");

    let symbols: HashSet<Location> = input
        .lines()
        .enumerate()
        .flat_map(|(row_num, line)| {
            match_symbol
                .find_iter(line)
                .map(move |symbol_match| Location {
                    x: symbol_match.start() as i32,
                    y: row_num as i32,
                })
        })
        .collect();

    let match_number = Regex::new(r"\d+").expect("Failed to compile regex");

    input
        .lines()
        .enumerate()
        .flat_map(|(row_num, line)| {
            match_number.find_iter(line).map(move |num_match| {
                Number::new(
                    num_match.start(),
                    num_match.end() - 1,
                    row_num,
                    num_match.as_str().parse().expect("Failed to parse number"),
                )
            })
        })
        .filter(|num| num.is_adjacent(&symbols))
        .fold(0, |total, num| total + num.number)
}

fn part_2(input: &str) -> usize {
    let match_gear = Regex::new(r"\*").expect("Failed to compile regex");

    let gears: HashSet<Location> = input
        .lines()
        .enumerate()
        .flat_map(|(row_num, line)| {
            match_gear
                .find_iter(line)
                .map(move |symbol_match| Location {
                    x: symbol_match.start() as i32,
                    y: row_num as i32,
                })
        })
        .collect();

    let match_number = Regex::new(r"\d+").expect("Failed to compile regex");

    let numbers: Vec<Number> = input
        .lines()
        .enumerate()
        .flat_map(|(row_num, line)| {
            match_number.find_iter(line).map(move |num_match| {
                Number::new(
                    num_match.start(),
                    num_match.end() - 1,
                    row_num,
                    num_match.as_str().parse().expect("Failed to parse number"),
                )
            })
        })
        .collect();

    let mut gears_to_numbers: HashMap<Location, HashSet<&Number>> = HashMap::new();

    for gear in gears {
        for x_off in -1..=1 {
            for y_off in -1..=1 {
                let location = Location {
                    x: gear.x + x_off,
                    y: gear.y + y_off,
                };
                if let Some(number) = numbers.iter().find(|num| num.is_at_location(&location)) {
                    gears_to_numbers
                        .entry(gear)
                        .or_insert_with(HashSet::new)
                        .insert(number);
                }
            }
        }
    }

    gears_to_numbers
        .iter()
        .filter(|(_, numbers)| numbers.len() == 2)
        .fold(0, |total, (_, numbers)| {
            total + numbers.into_iter().map(|num| num.number).product::<usize>()
        })
}
