use ring_algorithm::chinese_remainder_theorem;
use std::collections::HashMap;

fn main() {
    let input: Vec<&str> = include_str!("../../inputs/013.txt").lines().collect();

    println!("Part one: {}", part_one(&input));

    // this answer actually has the wrong sign and im not sure why 🤷‍♂️️
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &[&str]) -> usize {
    let earliest_departure = input[0].parse().unwrap();
    let bus_times: HashMap<usize, usize> = input[1]
        .split(',')
        .filter(|bus_time| bus_time != &"x")
        .map(|bus_time| bus_time.parse::<usize>().unwrap())
        .map(|bus_time| {
            let mut next_bus = bus_time;
            while next_bus < earliest_departure {
                next_bus += bus_time;
            }
            (bus_time, next_bus)
        })
        .collect();

    let min = bus_times.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    min.0 * (min.1 - earliest_departure)
}

fn part_two(input: &[&str]) -> isize {
    let expressions: HashMap<isize, isize> = input[1]
        .split(',')
        .enumerate()
        .filter(|(_, bus)| bus != &"x")
        .map(|(i, bus)| (i as isize, bus.parse::<isize>().unwrap()))
        .collect();

    let remainders: Vec<isize> = expressions.keys().cloned().collect();
    let modulos: Vec<isize> = expressions.values().cloned().collect();

    chinese_remainder_theorem(&remainders, &modulos).unwrap()
}
