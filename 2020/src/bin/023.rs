use std::{mem::MaybeUninit, time::Instant};

const NUM_CUPS: usize = 9;

fn main() {
    let input: Vec<u8> = include_bytes!("../../inputs/023.txt")
        .iter()
        .filter(|&b| b != &b'\n')
        .map(|&b| b - b'0')
        .collect();

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(&input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} in {:#?}", part_two(&input), start.elapsed());
}

fn part_one(input: &[u8]) -> String {
    let mut cups: [u8; NUM_CUPS + 1] = [0; NUM_CUPS + 1];

    input.iter().enumerate().for_each(|(i, cup)| {
        if i == NUM_CUPS - 1 {
            cups[*cup as usize] = input[0]
        } else {
            cups[*cup as usize] = input[i + 1]
        }
    });

    let mut current = input[0];

    for _ in 0..100 {
        // from current cup, pick up 3 cups clockwise from it
        let a = cups[current as usize];
        let b = cups[a as usize];
        let c = cups[b as usize];

        // find destination
        let mut destination: u8 = current;
        while [current, a, b, c].contains(&destination) {
            if destination == 1 {
                destination = NUM_CUPS as u8;
            } else {
                destination -= 1;
            }
        }

        // place cups immediately clockwise of destination
        cups[current as usize] = cups[c as usize];
        cups[c as usize] = cups[destination as usize];
        cups[destination as usize] = a;

        // change current to next in circle
        current = cups[current as usize];
    }

    let mut answer: usize = 0;
    let mut next = cups[1];
    while next != 1 {
        answer = answer * 10 + next as usize;
        next = cups[next as usize];
    }
    answer.to_string()
}

fn part_two(input: &[u8]) -> usize {
    // cup labels will need to be u32 now
    let input: Vec<u32> = input.iter().map(|x| *x as u32).collect();

    let mut cups: [u32; 1_000_001] = [0; 1_000_001];

    // find the last cup in the circle so far
    let last = input[1..].iter().fold(input[0], |prev, cup| {
        cups[prev as usize] = *cup;
        *cup
    });

    // start adding subsequent numbers on the circle after the last
    (input.len() + 1..1_000_000).for_each(|c| cups[c] = c as u32 + 1);
    cups[last as usize] = input.len() as u32 + 1;
    // wrap around to the start
    cups[1_000_000] = input[0];

    let mut current = input[0];

    // same as in part one but 10 million times
    for _ in 0..10_000_000 {
        let a = cups[current as usize];
        let b = cups[a as usize];
        let c = cups[b as usize];

        // find destination
        let mut destination: u32 = current;
        while [current, a, b, c].contains(&destination) {
            if destination == 1 {
                destination = 1_000_000_u32;
            } else {
                destination -= 1;
            }
        }

        cups[current as usize] = cups[c as usize];
        cups[c as usize] = cups[destination as usize];
        cups[destination as usize] = a;

        current = cups[current as usize];
    }

    let a = cups[1_usize] as usize;
    let b = cups[a as usize] as usize;
    (a * b) as usize
}
