use std::collections::HashSet;

use santas_little_helpers::coord::Coord;

fn main() {
    let input = include_str!("input.txt");
    let (map, guard) = parse(input);

    println!("Part 1: {}", part_1(map.clone(), guard));
    println!("Part 2: {}", part_2(map, guard));
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

fn facing_obstacle(&self, position: Coord, direction: Direction) -> bool {
    let new_pos = direction.apply(&position);
    if !self.in_bounds(new_pos) {
        return false;
    }

    match self.tiles[new_pos.1 as usize][new_pos.0 as usize] {
        b'#' => true,
        b'.' => false,
        _ => unreachable!(),
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

        guard.pos = forwards(&guard.pos, guard.facing);

        if !map.in_bounds(guard.pos) {
            break;
        }

        visited.insert(guard.pos);
    }

    visited.len()
}

fn part_2(mut map: Map, mut guard: Guard) -> usize {
    let guard_start = guard.pos;

    let mut initially_visited = HashSet::new();
    initially_visited.insert(guard.pos);

    loop {
        if map.facing_obstacle(guard.pos, guard.facing) {
            guard.facing = guard.facing.turn_right();
        }

        guard.pos = forwards(&guard.pos, guard.facing);

        if !map.in_bounds(guard.pos) {
            break;
        }

        initially_visited.insert(guard.pos);
    }

    initially_visited.remove(&guard_start);

    guard.pos = guard_start;
    guard.facing = Direction::Up;

    let mut valid_locations = 0;

    for &obstacle_location in initially_visited.iter() {
        let mut visited = HashSet::<(Coord, Direction)>::new();
        map.set_tile(obstacle_location, b'#');

        loop {
            while map.facing_obstacle(guard.pos, guard.facing) {
                guard.facing = guard.facing.turn_right();
            }

            guard.pos = forwards(&guard.pos, guard.facing);

            if !map.in_bounds(guard.pos) {
                break;
            }

            if visited.contains(&(guard.pos, guard.facing)) {
                valid_locations += 1;
                break;
            }
            visited.insert((guard.pos, guard.facing));
        }

        guard.pos = guard_start;
        guard.facing = Direction::Up;
        map.set_tile(obstacle_location, b'.');
        visited.clear();
    }

    valid_locations

    //let valid_locations = AtomicUsize::new(0);
    //initially_visited.par_iter().for_each(|&obstacle_location| {
    //    let mut this_guard = guard.clone();
    //    let mut this_map = map.clone();
    //    let mut visited = HashSet::<(Coord, Direction)>::new();
    //    this_map.set_tile(obstacle_location, b'#');
    //
    //    loop {
    //        while this_map.facing_obstacle(this_guard.pos, this_guard.facing) {
    //            this_guard.facing = this_guard.facing.turn_right();
    //        }
    //
    //        this_guard.pos = forwards(&this_guard.pos, this_guard.facing);
    //
    //        if !this_map.in_bounds(this_guard.pos) {
    //            break;
    //        }
    //
    //        if visited.contains(&(this_guard.pos, this_guard.facing)) {
    //            valid_locations.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    //            break;
    //        }
    //        visited.insert((this_guard.pos, this_guard.facing));
    //    }
    //});

    //valid_locations.load(std::sync::atomic::Ordering::Relaxed)
}

fn parse(input: &str) -> (Map, Guard) {
    let mut guard = Guard {
        pos: (0, 0),
        facing: Direction::Up,
    };

    let width = input.lines().nth(1).unwrap().bytes().len();
    let height = input.lines().count();

    let mut tiles = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, tile) in line.bytes().enumerate() {
            let coord = (x, y);
            match tile {
                b'#' => {
                    row.push(b'#');
                }
                b'.' => {
                    row.push(b'.');
                }
                b'^' => {
                    guard.pos = coord;
                    row.push(b'.');
                }
                _ => panic!("bad tile"),
            }
        }
        tiles.push(row);
    }

    let map = Map::new(tiles, width as usize, height as usize);

    (map, guard)
}
