use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use santas_little_helpers::coord;
use santas_little_helpers::{coord::Coord, grid::Grid};

fn main() {
    let input = include_str!("input.txt");

    let (grid, start, end) = parse(input);
    let distances = distances(&grid, start, end);

    println!("Part 1: {}", cheats(&distances, 2));
    println!("Part 2: {}", cheats(&distances, 20));
}

#[derive(PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
}

fn cheats(distances: &HashMap<Coord, usize>, allowed_time: usize) -> usize {
    let mut possible_cheats = 0;

    for ((pos1, cost1), (pos2, cost2)) in distances.iter().tuple_combinations() {
        let dist = pos1.manhattan_dist(pos2);
        let cost_saving = cost1.abs_diff(*cost2);

        if dist <= allowed_time && cost_saving >= dist + 100 {
            possible_cheats += 1;
        }
    }

    possible_cheats
}

fn parse(input: &str) -> (Grid<Tile>, Coord, Coord) {
    let mut start = coord!(0, 0);
    let mut end = coord!(0, 0);

    let mut grid = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, char) in line.bytes().enumerate() {
            match char {
                b'.' => row.push(Tile::Floor),
                b'#' => row.push(Tile::Wall),
                b'S' => {
                    start = coord!(x, y);
                    row.push(Tile::Floor);
                }
                b'E' => {
                    end = coord!(x, y);
                    row.push(Tile::Floor);
                }
                _ => panic!("bad char"),
            }
        }
        grid.push(row);
    }

    let grid = Grid::from_vecs(grid);

    (grid, start, end)
}

fn distances(grid: &Grid<Tile>, start: Coord, end: Coord) -> HashMap<Coord, usize> {
    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    queue.push_back((start, 0_usize));

    while let Some((pos, cost)) = queue.pop_front() {
        if distances.contains_key(&pos) {
            continue;
        }
        distances.insert(pos, cost);

        if pos == end {
            continue;
        }

        for neighbour in grid.matching_neighbours(pos, Tile::Floor) {
            queue.push_back((neighbour, cost + 1));
        }
    }

    distances
}
