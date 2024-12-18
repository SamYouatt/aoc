use std::collections::{HashSet, VecDeque};

use santas_little_helpers::coord;
use santas_little_helpers::coord::Coord;
use santas_little_helpers::grid::Grid;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let size = 71;
    let mut map = Grid::init(false, size, size);

    let coords = input.lines().take(1024).map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<isize>().unwrap();
        let y = y.parse::<isize>().unwrap();
        coord!(x, y)
    });

    for coord in coords {
        map.set(coord, true);
    }

    bfs(coord!(0, 0), &map, coord!(size - 1, size - 1))
}

fn bfs(start: Coord, map: &Grid<bool>, goal: Coord) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((pos, steps)) = queue.pop_front() {
        if pos == goal {
            return steps;
        }

        for next in map.neighbours(pos) {
            if visited.contains(&next) || *map.get(&next) {
                continue;
            }

            visited.insert(next);
            queue.push_back((next, steps + 1));
        }
    }

    unreachable!("found no path");
}
