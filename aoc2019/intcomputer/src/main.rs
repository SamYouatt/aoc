use intcomputer::{parse_tape, Computer};

fn main() {
    println!("Day 2 part 1: {}", day2());
}

fn day2() -> usize {
    let day2_input = include_str!("inputs/day2_input.txt");

    let mut tape = parse_tape(day2_input);
    tape[1] = 12;
    tape[2] = 2;

    let mut computer = Computer::new(&tape);
    computer.run();
    
    let final_tape = computer.dump_tape();

    final_tape[0]
}
