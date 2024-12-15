use std::collections::HashSet;

use santas_little_helpers::{coord, coord::Coord, directions::Direction, grid::Grid};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
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
