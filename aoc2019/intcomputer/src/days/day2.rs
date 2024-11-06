use std::sync::mpsc;

use itertools::Itertools;

use crate::{parse_tape, Computer};

pub fn part1(input: &str) -> usize {
    let mut tape = parse_tape(input);
    tape[1] = 12;
    tape[2] = 2;

    let (sender, receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, receiver, sender);
    computer.run();

    let final_tape = computer.dump_tape();

    final_tape[0] as usize
}

pub fn part2(input: &str) -> usize {
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
