use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use santas_little_helpers::{coord, coord::Coord, directions::Direction, grid::Grid};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
struct Current {
    pos: Coord,
    facing: Direction,
    total: usize,
}

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

impl Current {
    fn clockwise(self: &Current) -> Self {
        Self {
            pos: self.pos,
            facing: self.facing.turn_right(),
            total: self.total + 1000,
        }
    }

    fn counter_clockwise(self: &Current) -> Self {
        Self {
            pos: self.pos,
            facing: self.facing.turn_left(),
            total: self.total + 1000,
        }
    }

    fn mv(self: &Current) -> Self {
        Self {
            pos: self.pos + self.facing.delta(),
            facing: self.facing,
            total: self.total + 1,
        }
    }
}

impl Ord for Current {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total.cmp(&other.total)
    }
}

impl PartialOrd for Current {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.total.cmp(&other.total))
    }
}

fn part_1(input: &str) -> usize {
    let mut map = Vec::new();
    let mut start = coord!(0, 0);
    let mut end = coord!(0, 0);

    for (y, raw_line) in input.lines().enumerate() {
        let mut line = Vec::new();
        for (x, tile) in raw_line.bytes().enumerate() {
            match tile {
                b'#' => line.push(Tile::Wall),
                b'.' => line.push(Tile::Empty),
                b'S' => {
                    line.push(Tile::Empty);
                    start = coord!(x, y)
                }
                b'E' => {
                    line.push(Tile::Empty);
                    end = coord!(x, y)
                }
                _ => unreachable!(),
            }
        }
        map.push(line);
    }

    let map = Grid::from_vecs(map);
    let current = Current {
        pos: start,
        facing: Direction::Right,
        total: 0,
    };
    let mut best = usize::MAX;

    let mut heap = BinaryHeap::new();
    let mut seen = HashSet::new();
    heap.push(Reverse(current.mv()));
    heap.push(Reverse(current.clockwise()));
    heap.push(Reverse(current.counter_clockwise()));

    while let Some(Reverse(node)) = heap.pop() {
        if !seen.insert((node.pos, node.facing)) {
            continue;
        }

        if node.pos == end {
            best = node.total;
            break;
        }

        if map.get(&node.pos) == &Tile::Wall {
            continue;
        }

        heap.push(Reverse(node.mv()));
        heap.push(Reverse(node.clockwise()));
        heap.push(Reverse(node.counter_clockwise()));
    }

    best
}
