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

#[test]
fn example_that_makes_large_middle_num() {
    let input = "104,1125899906842624,99";
    let tape = parse_tape(input);
    let (_in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, in_receiver, out_sender);
    computer.run();

    assert_eq!(out_receiver.recv().expect("no output"), tape[1]); 
}

#[test]
fn example_that_outputs_16dig_number() {
    let input = "1102,34915192,34915192,7,4,7,99,0";
    let tape = parse_tape(input);
    let (_in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, in_receiver, out_sender);
    computer.run();

    assert!(out_receiver.recv().expect("no output") > 999999999999999); 
}
