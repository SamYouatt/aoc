use std::{sync::mpsc, thread};

use itertools::Itertools;

use crate::computer::{parse_tape, Computer};

pub fn part1(input: &str) -> usize {
    let tape = parse_tape(input);

    let mut max_signal = 0;

    for permutation in (0i64..5).permutations(5) {
        let (mut current_sender, mut current_receiver) = mpsc::channel();
        // Challenge says to feed A a 0 to start the process so hang onto this
        let first_sender = current_sender.clone();
        let mut amplifiers = Vec::new();

        for phase_setting in permutation.iter() {
            current_sender
                .send(*phase_setting)
                .expect("send should never close");

            let (next_sender, next_receiver) = mpsc::channel();

            let amplifier = Computer::load(&tape, current_receiver, next_sender.clone());

            amplifiers.push(amplifier);

            (current_receiver, current_sender) = (next_receiver, next_sender);
        }

        first_sender
            .send(0)
            .expect("first channel should never close");

        for mut amplifier in amplifiers {
            amplifier.run();
        }

        let circuit_signal = current_receiver.recv().expect("recv should never close");
        max_signal = max_signal.max(circuit_signal);
    }

    max_signal as usize
}

pub fn part2(input: &str) -> usize {
    let tape = parse_tape(input);
    let (results_send, results_recv) = mpsc::channel();

    for permutation in (5i64..10).permutations(5) {
        let (mut current_sender, mut current_receiver) = mpsc::channel();
        // Challenge says to feed A a 0 to start the process so hang onto this
        let first_sender = current_sender.clone();

        for (i, phase_setting) in permutation.iter().enumerate() {
            current_sender
                .send(*phase_setting)
                .expect("send should never close");

            let (mut next_sender, next_receiver) = mpsc::channel();

            // Final circuit needs to feed back to beginning
            if i == 4 {
                next_sender = first_sender.clone();
            }

            let mut amplifier = Computer::load(&tape, current_receiver, next_sender.clone());
            let results_send = results_send.clone();

            thread::spawn(move || {
                amplifier.run();

                // A's final position should have one message left on the queue - which is the
                // output for that, alternative here is to make the final sender for E be a
                // broadcast, or allow each computer to take multiple senders
                if i == 0 {
                    let unused_message = amplifier.receiver().recv();
                    results_send.send(unused_message).expect("results send should not close");
                }
            });

            (current_receiver, current_sender) = (next_receiver, next_sender);
        }

        // Kickstart the circuit
        first_sender
            .send(0)
            .expect("first sender should never close");
    }

    drop(results_send);
    results_recv
        .iter()
        .map(|x| x.expect("results recv should not close"))
        .max()
        .expect("no values output") as usize
}
