use intcomputer::{parse_tape, reader::StdInReader, writer::StdOutWriter, Computer};
use itertools::Itertools;

fn main() {
    let day2_input = include_str!("inputs/day2_input.txt");
    println!("Day 2 part 1: {}", day2_part1(day2_input));
    println!("Day 2 part 2: {}", day2_part2(day2_input));
}

fn day2_part1(input: &str) -> usize {
    let mut tape = parse_tape(input);
    tape[1] = 12;
    tape[2] = 2;

    let mut computer = Computer::load(&tape, StdInReader, StdOutWriter);
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

        let mut computer = Computer::load(&tape, StdInReader, StdOutWriter);
        computer.run();

        let final_tape = computer.dump_tape();
        if final_tape[0] == goal {
            return (100 * noun + verb) as usize;
        }
    }

    unreachable!("didn't find answer");
}
