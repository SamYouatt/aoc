use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use santas_little_helpers::{coord, coord::Coord, directions::Direction, grid::Grid};

fn main() {
    let input = include_str!("input.txt");
    let (p1, p2) = both_parts(input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Node {
    pos: Coord,
    facing: Direction,
    total: usize,
    path: Vec<Coord>,
}

#[derive(PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

impl Node {
    fn clockwise(self: &Node) -> Self {
        Self {
            pos: self.pos,
            facing: self.facing.turn_right(),
            total: self.total + 1000,
            path: self.path.clone(),
        }
    }

    fn counter_clockwise(self: &Node) -> Self {
        Self {
            pos: self.pos,
            facing: self.facing.turn_left(),
            total: self.total + 1000,
            path: self.path.clone(),
        }
    }

    fn mv(self: &Node) -> Self {
        let new_pos = self.pos + self.facing.delta();
        let mut path = self.path.clone();
        path.push(new_pos);

        Self {
            pos: new_pos,
            facing: self.facing,
            total: self.total + 1,
            path,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.total.cmp(&other.total)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.total.cmp(&other.total))
    }
}

fn both_parts(input: &str) -> (usize, usize) {
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
    let current = Node {
        pos: start,
        facing: Direction::Right,
        total: 0,
        path: vec![start],
    };
    let mut best = usize::MAX;

    let mut heap = BinaryHeap::new();
    let mut seen = HashMap::new();
    let mut benches = HashSet::new();
    heap.push(Reverse(current.mv()));
    heap.push(Reverse(current.clockwise()));
    heap.push(Reverse(current.counter_clockwise()));

    while let Some(Reverse(node)) = heap.pop() {
        if node.pos == end {
            if node.total > best {
                break;
            }

            best = node.total;
            benches.extend(node.path);
            continue;
        }

        if !seen.contains_key(&(node.pos, node.facing)) {
            seen.insert((node.pos, node.facing), node.total);
        } else {
            if seen.get(&(node.pos, node.facing)).unwrap() < &node.total {
                continue;
            }
        }

        if map.get(&node.pos) == &Tile::Wall {
            continue;
        }

        heap.push(Reverse(node.mv()));
        heap.push(Reverse(node.clockwise()));
        heap.push(Reverse(node.counter_clockwise()));
    }

    (best, benches.len())
}
