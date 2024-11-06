use std::sync::mpsc;

use intcomputer::{parse_tape, Computer};

#[test]
fn beefy_example_input_below_8() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let tape = parse_tape(input);
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, in_receiver, out_sender.clone());

    in_sender.send(3).unwrap();
    computer.run();

    assert_eq!(out_receiver.recv().unwrap(), 999);
}

#[test]
fn beefy_example_input_equals_8() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let tape = parse_tape(input);
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, in_receiver, out_sender.clone());

    in_sender.send(8).unwrap();
    computer.run();

    assert_eq!(out_receiver.recv().unwrap(), 1000);
}

#[test]
fn beefy_example_input_greater_than_8() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let tape = parse_tape(input);
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, in_receiver, out_sender.clone());

    in_sender.send(19).unwrap();
    computer.run();

    assert_eq!(out_receiver.recv().unwrap(), 1001);
}
