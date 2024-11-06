use std::sync::mpsc;

use crate::{parse_tape, Computer};

pub fn part1(input: &str) -> usize {
    let tape = parse_tape(input);
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, in_receiver, out_sender.clone());
    in_sender.send(1).expect("in sender should never close");
    computer.run();

    drop(out_sender);
    out_receiver
        .iter()
        .next()
        .expect("no values output") as usize
}
