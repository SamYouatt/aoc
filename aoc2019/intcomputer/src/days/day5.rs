use std::sync::mpsc;

use crate::computer::{parse_tape, Computer};

pub fn both_parts(input: &str, user_input: i64) -> usize {
    let tape = parse_tape(input);

    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, in_receiver, out_sender.clone());
    in_sender.send(user_input).expect("send should never close");
    computer.run();

    out_receiver.recv().expect("recv should never close") as usize
}
