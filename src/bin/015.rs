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

    // lookup times are much faster for arrays than for hashmaps but we can't have an array of 30 million u32 as this would
    // use (30,000,000 * 4 = 120,000,000) 120GB of RAM
    //
    // however not every number is going to appear in the pattern, lower numbers are more likely to appear, they are denser
    // by splitting the storage space between two types of storage we can improve performance
    // an array can be used for the lower, denser, numbers with a hash map for the sparse values
    //
    // the ratio is going to be based on the amount of memory available
    // with the current ratio of / 15 the array will contain 2,000,000 u32 values
    // a u32 value is made of 4 bytes, so this will use 8,000,000 bytes (8GibbyBytes)
    // array must be stored contigously in memory so even though my system has 32GB of RAM it is unlikely
    // that this amount will be available contigously so the program will encounter stack overflow despite theoretically
    // being able to use more resources.
    //
    // it is hard to know the amount of memory that will be needed for the hashmap which will hold the remaining values
    // without being able to predict what numbers will actually appear. As a conservative estimate that doesn't degrade
    // performance I have it set to 3,000 entries which doesn't cause any problems for the algorithm and uses as little
    // entries as possible to speed up the hashmap
    //
    // with the current values it sees an increase of around 5x speed, from 2.3s to 380ms compared with only hashmap

    const RATIO: u32 = 30_000_000 / 15;
    let mut sparse_numbers: HashMap<u32, u32> = HashMap::with_capacity(30_000_000 / 10000);
    let mut dense_numbers: [u32; RATIO as usize] = [0; RATIO as usize];

    input
        .iter()
        .enumerate()
        .for_each(|(i, &num)| dense_numbers[num as usize] = i as u32 + 1);

    for i in input.len() as u32 + 1..30_000_000 {
        if current < RATIO {
            let num = &mut dense_numbers[current as usize];
            current = if *num == 0 { 0 } else { i - *num };
            *num = i;
        } else {
            match sparse_numbers.entry(current) {
                Entry::Occupied(mut occupied) => current = i - occupied.insert(i),
                Entry::Vacant(vacant) => {
                    vacant.insert(i);
                    current = 0;
                }
            }
        }
    }

    current
}

#[test]
fn test_part_two() {
    let input = "0,3,6";

    assert_eq!(part_two(input), 175594);
}
