use std::{
    collections::{hash_map::Entry, HashMap},
    time::Instant,
};

fn main() {
    let input = include_str!("../../inputs/015.txt");

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} in {:#?}", part_two(input), start.elapsed());
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

fn part_two(input: &str) -> u32 {
    let mut input: Vec<_> = input
        .trim_end()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut current = input.pop().expect("vector is empty");
    let mut numbers: HashMap<u32, u32> = HashMap::with_capacity(30_000_000);

    input
        .iter()
        .enumerate()
        .for_each(|(i, &num)| match numbers.entry(num) {
            Entry::Occupied(mut occupied) => {
                occupied.insert(i as u32 + 1);
            }
            Entry::Vacant(vacant) => {
                vacant.insert(i as u32 + 1);
            }
        });

    for i in input.len() as u32 + 1..30_000_000 {
        match numbers.entry(current) {
            Entry::Occupied(mut occupied) => current = i - occupied.insert(i),
            Entry::Vacant(vacant) => {
                vacant.insert(i);
                current = 0;
            }
        }
    }

    current

    // let mut current = input.pop().unwrap();
    // let mut numbers: [u32; 30_000_000] = [0; 30_000_000];

    // for i in input.len() as u32 + 1..30_000_000 {
    //     let num = &mut numbers[current as usize];
    //     current = if *num == 0 { 0 } else { i - *num };
    //     *num = i;
    // }

    // input
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i, &num)| numbers[num as usize] = i as u32 + 1);

    // current
}

#[test]
fn test_part_two() {
    let input = "0,3,6";

    assert_eq!(part_two(input), 175594);
}
