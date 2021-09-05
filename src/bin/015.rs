use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("../../inputs/015.txt");

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let mut numbers = input
        .trim_end()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<usize>>();

    while numbers.len() < 2020 {
        let previous = numbers.last().expect("no last number");

        // find the first instance of the number being the same,
        // the vector has been reverseved first so the first instance will actually have been near the end
        // if there is a match then push the index it was found (the time since it was last said)
        // or push 0 because it hasn't been said yet
        match numbers
            .iter()
            .rev()
            .enumerate()
            .skip(1)
            .find(|(_, number)| number == &previous)
        {
            Some((index, _)) => numbers.push(index),
            _ => numbers.push(0),
        }
    }

    *numbers.last().expect("no last number")
}
