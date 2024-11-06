use std::sync::mpsc;

use intcomputer::{parse_tape, Computer};

#[test]
fn first_input() {
    let input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let tape = parse_tape(input);
    let (sender, receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, receiver, sender);

    computer.run();

    let final_tape = computer.dump_tape();
    assert_eq!(
        final_tape,
        vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
}

#[test]
fn third_input() {
    let input = "2,3,0,3,99";
    let tape = parse_tape(input);
    let (sender, receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, receiver, sender);

    computer.run();

    let final_tape = computer.dump_tape();
    assert_eq!(final_tape, vec![2, 3, 0, 6, 99]);
}

#[test]
fn fourth_input() {
    let input = "2,4,4,5,99,0";
    let tape = parse_tape(input);
    let (sender, receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, receiver, sender);

    computer.run();

    let final_tape = computer.dump_tape();
    assert_eq!(final_tape, vec![2, 4, 4, 5, 99, 9801]);
}

#[test]
fn fifth_input() {
    let input = "1,1,1,4,99,5,6,0,99";
    let tape = parse_tape(input);
    let (sender, receiver) = mpsc::channel();

    let mut computer = Computer::load(&tape, receiver, sender);

    computer.run();

    let final_tape = computer.dump_tape();
    assert_eq!(final_tape, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}
