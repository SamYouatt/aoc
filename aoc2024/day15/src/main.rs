use std::collections::{HashSet, VecDeque};

use santas_little_helpers::{
    coord,
    coord::{Coord, Delta},
    directions::Direction,
    grid::Grid,
};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Floor,
}

fn part_1(input: &str) -> usize {
    let (map_raw, instructions) = input.split_once("\n\n").unwrap();

    let mut map = Vec::new();
    let mut robot = coord!(0, 0);
    let mut boxes = HashSet::new();

    for (y, line_raw) in map_raw.lines().enumerate() {
        let mut line = Vec::new();
        for (x, tile) in line_raw.trim().bytes().enumerate() {
            match tile {
                b'#' => {
                    line.push(Tile::Wall);
                }
                b'.' => {
                    line.push(Tile::Floor);
                }
                b'O' => {
                    line.push(Tile::Floor);
                    boxes.insert(coord!(x, y));
                }
                b'@' => {
                    line.push(Tile::Floor);
                    robot = coord!(x, y);
                }
                _ => unreachable!("bad tile"),
            }
        }
        map.push(line);
    }

    let map = Grid::from_vecs(map);

    let instructions = instructions
        .bytes()
        .filter(|b| *b != b'\n')
        .map(to_instruction);

    for instruction in instructions {
        if let Some(space) = space_in_direction(robot, instruction, &boxes, &map) {
            let delta = instruction.delta();
            robot = robot + delta;

            let mut next = robot;

            while next != space {
                boxes.insert(next + delta);
                next = next + delta;
            }

            // Remove trailing box if its the last one in the line
            if !boxes.contains(&(robot - delta)) {
                boxes.remove(&robot);
            }
        }
    }

    boxes.iter().map(|b| (100 * b.y + b.x) as usize).sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile2 {
    Wall,
    Floor,
    Robot,
    BoxL,
    BoxR,
}

fn part_2(input: &str) -> usize {
    let (map_raw, instructions) = input.split_once("\n\n").unwrap();

    let mut map = Vec::new();
    let mut robot = coord!(0, 0);

    let (mut x, mut y) = (0, 0);
    for line_raw in map_raw.lines() {
        x = 0;

        let mut line = Vec::new();
        for tile in line_raw.trim().bytes() {
            match tile {
                b'#' => {
                    line.push(Tile2::Wall);
                    line.push(Tile2::Wall);
                }
                b'.' => {
                    line.push(Tile2::Floor);
                    line.push(Tile2::Floor);
                }
                b'O' => {
                    line.push(Tile2::BoxL);
                    line.push(Tile2::BoxR);
                }
                b'@' => {
                    line.push(Tile2::Robot);
                    line.push(Tile2::Floor);
                    robot = coord!(x, y);
                }
                _ => unreachable!("bad tile"),
            }

            x += 2;
        }

        y += 1;
        map.push(line);
    }

    let mut map = Grid::from_vecs(map);

    let instructions = instructions
        .bytes()
        .filter(|b| *b != b'\n')
        .map(to_instruction);

    for instruction in instructions {
        match instruction {
            Direction::Up | Direction::Down => {
                shove_vertical(&mut robot, instruction.delta(), &mut map);
            }
            Direction::Left | Direction::Right => {
                shove_horiz(&mut robot, instruction.delta(), &mut map);
            }
        }
    }

    let mut total = 0;
    for (y, line) in map.grid.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if tile == &Tile2::BoxL {
                total += 100 * y + x;
            }
        }
    }

    total
}

fn shove_horiz(robot: &mut Coord, delta: Delta, map: &mut Grid<Tile2>) {
    let mut next = *robot + delta;
    let mut tiles_covered = 2;

    loop {
        match map.get(&next) {
            Tile2::Wall => return,
            Tile2::Floor => break,
            _ => {
                next = next + delta;
                tiles_covered += 1;
            }
        }
    }

    let mut prev = Tile2::Floor;
    let mut pos = *robot;

    for _ in 0..tiles_covered {
        std::mem::swap(&mut prev, &mut map.get_mut(pos));
        pos = pos + delta;
    }

    *robot = *robot + delta;
}

fn shove_vertical(robot: &mut Coord, delta: Delta, map: &mut Grid<Tile2>) {
    let mut work = VecDeque::from(vec![*robot]);
    let mut tiles_to_update = vec![*robot];
    let mut seen = HashSet::new();

    while let Some(next_work) = work.pop_front() {
        let next = next_work + delta;

        let other_next = match map.get(&next) {
            Tile2::BoxL => next + Direction::Right.delta(),
            Tile2::BoxR => next + Direction::Left.delta(),
            Tile2::Wall => return,
            _ => continue,
        };

        if seen.insert(next) {
            work.push_back(next);
            tiles_to_update.push(next);
        }

        if seen.insert(other_next) {
            work.push_back(other_next);
            tiles_to_update.push(other_next);
        }
    }

    for coord in tiles_to_update.iter().rev() {
        map.set(*coord + delta, *map.get(&coord));
        map.set(*coord, Tile2::Floor);
    }

    *robot = *robot + delta;
}

fn to_instruction(byte: u8) -> Direction {
    match byte {
        b'>' => Direction::Right,
        b'v' => Direction::Down,
        b'<' => Direction::Left,
        b'^' => Direction::Up,
        _ => unreachable!("bad instruction"),
    }
}

fn space_in_direction(
    start: Coord,
    direction: Direction,
    boxes: &HashSet<Coord>,
    map: &Grid<Tile>,
) -> Option<Coord> {
    let mut next = start + direction.delta();

    loop {
        if map.get(&next) == &Tile::Wall {
            return None;
        }

        if !boxes.contains(&next) {
            return Some(next);
        }

        next = next + direction.delta();
    }
}

fn print_grid(map: &Grid<Tile2>) {
    for line in map.grid.iter() {
        for char in line.iter() {
            match char {
                Tile2::Wall => print!("#"),
                Tile2::Floor => print!("."),
                Tile2::Robot => print!("@"),
                Tile2::BoxL => print!("["),
                Tile2::BoxR => print!("]"),
            }
        }
        print!("\n");
    }
}

fn assert_grid(map: &Grid<Tile2>) {
    for line in map.grid.iter() {
        let mut string = String::new();
        for char in line.iter() {
            match char {
                Tile2::Wall => string.push_str("#"),
                Tile2::Floor => string.push_str("."),
                Tile2::Robot => string.push_str("@"),
                Tile2::BoxL => string.push_str("["),
                Tile2::BoxR => string.push_str("]"),
            }
        }

        if string.contains("]]") || string.contains(".]") || string.contains("[[") || string.contains("[.") {
            panic!()
        }
    }
}
