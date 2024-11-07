use clap::Parser;
use intcomputer::days::*;

#[derive(Parser)]
#[command(name = "AoC Intcomputer")]
struct Cli {
    #[arg(short, long, value_parser = parse_day)]
    /// The day of the month (1-24)
    day: u8,

    #[arg(short, long, value_parser = parse_part)]
    /// The part of the challenge (1 or 2)
    part: u8,
}

fn parse_day(day: &str) -> Result<u8, String> {
    Ok(day.parse().expect("day should be between 1 and 25"))
}

fn parse_part(part: &str) -> Result<u8, String> {
    Ok(part.parse().expect("part should be 1 or 2"))
}

fn main() {
    let args = Cli::parse();

    let day2_input = include_str!("inputs/day2_input.txt");
    let day5_input = include_str!("inputs/day5_input.txt");
    let day7_input = include_str!("inputs/day7_input.txt");
    let day9_input = include_str!("inputs/day9_input.txt");
    let day11_input = include_str!("inputs/day11_input.txt");
    let day13_input = include_str!("inputs/day13_input.txt");

    match (args.day, args.part) {
        (2, 1) => println!("Day 2 part 1: {}", day2::part1(day2_input)),
        (2, 2) => println!("Day 2 part 2: {}", day2::part2(day2_input)),
        (5, 1) => println!("Day 5 part 1: {}", day5::both_parts(day5_input, 1)),
        (5, 2) => println!("Day 5 part 2: {}", day5::both_parts(day5_input, 5)),
        (7, 1) => println!("Day 7 part 1: {}", day7::part1(day7_input)),
        (7, 2) => println!("Day 7 part 2: {}", day7::part2(day7_input)),
        (9, 1) => println!("Day 9 part 1: {}", day9::part1(day9_input)),
        (9, 2) => println!("Day 9 part 2: {}", day9::part2(day9_input)),
        (11, 1) => println!("Day 11 part 1: {}", day11::part1(day11_input)),
        (11, 2) => {
            println!("Day 11 part 2...");
            day11::part2(day11_input);
        }
        (13, 1) => println!("Day 13 part 1: {}", day13::part1(day13_input)),
        (13, 2) => println!("Day 13 part 2: {}", day13::part2(day13_input)),
        _ => eprintln!("Pick a proper day and part fool"),
    }
}
