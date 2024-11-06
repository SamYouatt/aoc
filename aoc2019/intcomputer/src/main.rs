use std::{sync::mpsc, thread};

use clap::Parser;
use intcomputer::{parse_tape, Computer};
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
    let day7_input = include_str!("inputs/day7_input.txt");

    match (args.day, args.part) {
        (2, 1) => println!("Day 2 part 1: {}", day2_part1(day2_input)),
        (2, 2) => println!("Day 2 part 2: {}", day2_part2(day2_input)),
        (5, 1) => println!("Day 5 part 1: {}", day5(day5_input, 1)),
        (5, 2) => println!("Day 5 part 2: {}", day5(day5_input, 5)),
        (7, 1) => println!("Day 7 part 1: {}", day7_part1(day7_input)),
        (7, 2) => println!("Day 7 part 2: {}", day7_part2(day7_input)),
        _ => eprintln!("Pick a proper day and part fool"),
    }
}

fn day2_part1(input: &str) -> usize {
    let mut tape = parse_tape(input);
    tape[1] = 12;
    tape[2] = 2;

    let (sender, receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, receiver, sender);
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

        let (sender, receiver) = mpsc::channel();

        let mut computer = Computer::load(&tape, receiver, sender);
        computer.run();

        let final_tape = computer.dump_tape();
        if final_tape[0] == goal {
            return (100 * noun + verb) as usize;
        }
    }

    unreachable!("didn't find answer");
}

fn day5(input: &str, user_input: i64) -> usize {
    let tape = parse_tape(input);

    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, in_receiver, out_sender.clone());
    in_sender.send(user_input).expect("send should never close");
    computer.run();

    out_receiver.recv().expect("recv should never close") as usize
}

fn day7_part1(input: &str) -> usize {
    let tape = parse_tape(input);

    let mut max_signal = 0;

    for permutation in (0i64..5).permutations(5) {
        let (mut current_sender, mut current_receiver) = mpsc::channel();
        // Challenge says to feed A a 0 to start the process so hang onto this
        let first_sender = current_sender.clone();
        let mut amplifiers = Vec::new();

        for phase_setting in permutation.iter() {
            current_sender
                .send(*phase_setting)
                .expect("send should never close");

            let (next_sender, next_receiver) = mpsc::channel();

            let amplifier = Computer::load(&tape, current_receiver, next_sender.clone());

            amplifiers.push(amplifier);

            (current_receiver, current_sender) = (next_receiver, next_sender);
        }

        first_sender
            .send(0)
            .expect("first channel should never close");

        for mut amplifier in amplifiers {
            amplifier.run();
        }

        let circuit_signal = current_receiver.recv().expect("recv should never close");
        max_signal = max_signal.max(circuit_signal);
    }

    max_signal as usize
}

fn day7_part2(input: &str) -> usize {
    let tape = parse_tape(input);
    let (results_send, results_recv) = mpsc::channel();

    for permutation in (5i64..10).permutations(5) {
        let (mut current_sender, mut current_receiver) = mpsc::channel();
        // Challenge says to feed A a 0 to start the process so hang onto this
        let first_sender = current_sender.clone();

        for (i, phase_setting) in permutation.iter().enumerate() {
            current_sender
                .send(*phase_setting)
                .expect("send should never close");

            let (mut next_sender, next_receiver) = mpsc::channel();

            // Final circuit needs to feed back to beginning
            if i == 4 {
                next_sender = first_sender.clone();
            }

            let mut amplifier = Computer::load(&tape, current_receiver, next_sender.clone());
            let results_send = results_send.clone();

            thread::spawn(move || {
                amplifier.run();

                // A's final position should have one message left on the queue - which is the
                // output for that, alternative here is to make the final sender for E be a
                // broadcast, or allow each computer to take multiple senders
                if i == 0 {
                    let unused_message = amplifier.receiver().recv();
                    results_send.send(unused_message).expect("results send should not close");
                }
            });

            (current_receiver, current_sender) = (next_receiver, next_sender);
        }

        // Kickstart the circuit
        first_sender
            .send(0)
            .expect("first sender should never close");
    }

    drop(results_send);
    results_recv
        .iter()
        .map(|x| x.expect("results recv should not close"))
        .max()
        .expect("no values output") as usize
}
