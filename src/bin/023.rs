use std::time::Instant;

const NUM_CUPS: usize = 9;

fn main() {
    let input: Vec<u8> = include_bytes!("../../inputs/023.txt")
        .iter()
        .filter(|&b| b != &b'\n')
        .map(|&b| b - b'0')
        .collect();

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(&input), start.elapsed());
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
