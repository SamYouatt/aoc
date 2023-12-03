use std::collections::HashSet;

use regex::Regex;

#[derive(Debug)]
struct Number {
    occupies: Vec<Location>,
    number: usize,
}

impl Number {
    fn new(start: usize, end: usize, row: usize, number: usize) -> Number {
        let start_i = start.try_into().expect("Failed to cast");
        let end_i = end.try_into().expect("Failed to cast");
        let row_i = row.try_into().expect("Failed to cast");

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
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Location {
    x: i32,
    y: i32,
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {}", answer1);
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
                    x: symbol_match.start().try_into().expect("Failed to cast"),
                    y: row_num.try_into().expect("Failed to cast"),
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
