use clap::Parser;
use intcomputer::{parse_tape, reader::StdInReader, writer::StdOutWriter, Computer};
use itertools::Itertools;

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

    match (args.day, args.part) {
        (2, 1) => println!("Day 2 part 1: {}", day2_part1(day2_input)),
        (2, 2) => println!("Day 2 part 2: {}", day2_part2(day2_input)),
        (5, 1) | (5, 2) => day5(day5_input),
        _ => eprintln!("Pick a proper day and part fool"),
    }
}

fn day2_part1(input: &str) -> usize {
    let mut tape = parse_tape(input);
    tape[1] = 12;
    tape[2] = 2;

    let mut reader = StdInReader;
    let mut writer = StdOutWriter;

    let mut computer = Computer::load(&tape, &mut reader, &mut writer);
    computer.run();

    let final_tape = computer.dump_tape();

    final_tape[0] as usize
}

fn day2_part2(input: &str) -> usize {
    let goal = 19690720;

    let mut tape = parse_tape(input);

    for (noun, verb) in (0..=99).cartesian_product(0..=99) {
        tape[1] = noun;
        tape[2] = verb;

        let mut reader = StdInReader;
        let mut writer = StdOutWriter;

        let mut computer = Computer::load(&tape, &mut reader, &mut writer);
        computer.run();

        let final_tape = computer.dump_tape();
        if final_tape[0] == goal {
            return (100 * noun + verb) as usize;
        }
    }

    unreachable!("didn't find answer");
}

fn day5(input: &str) {
    let tape = parse_tape(input);
    let mut reader = StdInReader;
    let mut writer = StdOutWriter;

    let mut computer = Computer::load(&tape, &mut reader, &mut writer);
    computer.run();
}
