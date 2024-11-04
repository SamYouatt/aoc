use intcomputer::{parse_tape, Computer};

fn main() {
    let day2_input = include_str!("inputs/day2_input.txt");

    let tape = parse_tape(day2_input);

    let computer = Computer::new(&tape);

    todo!()
}
