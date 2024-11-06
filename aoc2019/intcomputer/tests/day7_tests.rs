use std::sync::mpsc;

use intcomputer::{parse_tape, Computer};

#[test]
fn example_that_produces_self() {
    let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let tape = parse_tape(input);
    let (sender, receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, receiver, sender);

    computer.run();

    assert!(computer.dump_tape().starts_with(&tape));
}
