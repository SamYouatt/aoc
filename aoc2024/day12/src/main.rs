use santa_claws::coord;
use santa_claws::coord::Coord;
use std::collections::{HashSet, VecDeque};

use santa_claws::grid::Grid;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let map: Grid<char> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>()
        .into();

    let mut visited: HashSet<Coord> = HashSet::new();

    let mut cost = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = coord!(x, y);
            if !visited.contains(&pos) {
                let (area, perim) = build_region(pos, &map, &mut visited);
                cost += area * perim;
            }
        }
    }

    cost
}

fn build_region(start: Coord, map: &Grid<char>, visited: &mut HashSet<Coord>) -> (usize, usize) {
    let mut work = VecDeque::new();
    let mut region = HashSet::new();
    let mut perimeter = 0;

    work.push_back(start);
    visited.insert(start);

    while let Some(next_loc) = work.pop_front() {
        region.insert(next_loc);

        let neighbours = map.matching_neighbours(next_loc);
        perimeter += 4 - neighbours.len();

        for neighbour in neighbours.into_iter() {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                work.push_back(neighbour);
            }
        }
    }

    (region.len(), perimeter)
}
