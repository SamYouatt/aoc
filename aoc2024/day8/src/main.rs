use itertools::iproduct;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut antennas = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                c => antennas
                    .entry(c)
                    .or_insert(vec![])
                    .push((x as isize, y as isize)),
            }
        }
    }

    let width = input.lines().nth(1).unwrap().chars().count();
    let height = input.lines().count();

    let mut antinodes = HashSet::new();
    for (_c, locations) in antennas.iter() {
        for (pos1, pos2) in iproduct!(locations, locations) {
            if pos1 == pos2 {
                continue;
            }

            let delta_x = pos2.0 - pos1.0;
            let delta_y = pos2.1 - pos1.1;

            let new_x = pos2.0 + delta_x;
            let new_y = pos2.1 + delta_y;

            if new_x < 0 || new_x >= width as isize || new_y < 0 || new_y >= height as isize {
                continue;
            }

            antinodes.insert((new_x, new_y));
        }
    }

    antinodes.len()
}

fn part_2(input: &str) -> usize {
    let mut antennas = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {}
                c => antennas
                    .entry(c)
                    .or_insert(vec![])
                    .push((x as isize, y as isize)),
            }
        }
    }

    let width = input.lines().nth(1).unwrap().chars().count();
    let height = input.lines().count();

    let mut antinodes = HashSet::new();
    for (_c, locations) in antennas.iter() {
        for (pos1, pos2) in iproduct!(locations, locations) {
            if pos1 == pos2 {
                continue;
            }
            antinodes.insert(*pos1);
            antinodes.insert(*pos2);

            let delta_x = pos2.0 - pos1.0;
            let delta_y = pos2.1 - pos1.1;

            let mut new_x = pos2.0 + delta_x;
            let mut new_y = pos2.1 + delta_y;

            while new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize {
                antinodes.insert((new_x, new_y));
                new_x += delta_x;
                new_y += delta_y;
            }
        }
    }

    antinodes.len()
}
