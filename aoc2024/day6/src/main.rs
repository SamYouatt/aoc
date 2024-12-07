use std::{
    collections::{HashMap, HashSet},
    sync::atomic::AtomicUsize,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("input.txt");
    let (map, guard) = parse(input);

    println!("Part 1: {}", part_1(map.clone(), guard));
    println!("Part 2: {}", part_2(map, guard));
}

#[derive(Clone)]
enum Tile {
    Obstacle,
    Free,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn apply_delta(self, delta: (isize, isize)) -> Self {
        Self {
            x: self.x + delta.0,
            y: self.y + delta.1,
        }
    }

    fn forwards(self, direction: Direction) -> Self {
        self.apply_delta(direction.delta())
    }
}

#[derive(Clone)]
struct Map {
    tiles: HashMap<Coord, Tile>,
    width: isize,
    height: isize,
}

impl Map {
    fn new(tiles: HashMap<Coord, Tile>, width: isize, height: isize) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

    fn in_bounds(&self, coord: Coord) -> bool {
        coord.x >= 0 && coord.x < self.width && coord.y >= 0 && coord.y < self.height
    }

    fn facing_obstacle(&self, position: Coord, direction: Direction) -> bool {
        let new_pos = position.apply_delta(direction.delta());

        match self.tiles.get(&new_pos) {
            Some(Tile::Obstacle) => true,
            Some(Tile::Free) => false,
            None => false,
        }
    }

    fn set_tile(&mut self, position: Coord, tile: Tile) {
        self.tiles.insert(position, tile);
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    pos: Coord,
    facing: Direction,
}

fn part_1(map: Map, mut guard: Guard) -> usize {
    let mut visited = HashSet::new();
    visited.insert(guard.pos);

    loop {
        if map.facing_obstacle(guard.pos, guard.facing) {
            guard.facing = guard.facing.turn_right();
        }

        guard.pos = guard.pos.forwards(guard.facing);

        if !map.in_bounds(guard.pos) {
            break;
        }

        visited.insert(guard.pos);
    }

    visited.len()
}

fn part_2(map: Map, mut guard: Guard) -> usize {
    let guard_start = guard.pos;

    let mut initially_visited = HashSet::new();
    initially_visited.insert(guard.pos);

    loop {
        if map.facing_obstacle(guard.pos, guard.facing) {
            guard.facing = guard.facing.turn_right();
        }

        guard.pos = guard.pos.forwards(guard.facing);

        if !map.in_bounds(guard.pos) {
            break;
        }

        initially_visited.insert(guard.pos);
    }

    initially_visited.remove(&guard_start);

    guard.pos = guard_start;
    guard.facing = Direction::Up;

    let valid_locations = AtomicUsize::new(0);
    initially_visited.par_iter().for_each(|&obstacle_location| {
        let mut this_guard = guard.clone();
        let mut this_map = map.clone();
        let mut visited = HashSet::<(Coord, Direction)>::new();
        this_map.set_tile(obstacle_location, Tile::Obstacle);

        loop {
            while this_map.facing_obstacle(this_guard.pos, this_guard.facing) {
                this_guard.facing = this_guard.facing.turn_right();
            }

            this_guard.pos = this_guard.pos.forwards(this_guard.facing);

            if !this_map.in_bounds(this_guard.pos) {
                break;
            }

            if visited.contains(&(this_guard.pos, this_guard.facing)) {
                valid_locations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                break;
            }
            visited.insert((this_guard.pos, this_guard.facing));
        }
    });

    valid_locations.load(std::sync::atomic::Ordering::Relaxed)
}

fn parse(input: &str) -> (Map, Guard) {
    let mut tiles = HashMap::new();
    let mut guard = Guard {
        pos: Coord::new(0, 0),
        facing: Direction::Up,
    };

    let width = input.lines().nth(1).unwrap().bytes().len();
    let height = input.lines().count();

    for (y, line) in input.lines().enumerate() {
        for (x, tile) in line.bytes().enumerate() {
            let coord = Coord::new(x as isize, y as isize);
            match tile {
                b'#' => {
                    tiles.insert(coord, Tile::Obstacle);
                }
                b'.' => {
                    tiles.insert(coord, Tile::Free);
                }
                b'^' => {
                    guard.pos = coord;
                    tiles.insert(coord, Tile::Free);
                }
                _ => panic!("bad tile"),
            }
        }
    }

    let map = Map::new(tiles, width as isize, height as isize);

    (map, guard)
}
