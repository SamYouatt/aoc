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
            println!("Can move {:?}", instruction);
            println!("Found space {:?}", space);
            let delta = instruction.delta();
            robot = robot + delta;

            let mut next = robot; // Robot's new position - possibly a box
            while next != space {
                println!("shifting {:?}", next);
                boxes.insert(next + delta);
                if !boxes.contains(&(next - delta)) {
                    println!("last in line so removing behind {:?}", next);
                    boxes.remove(&robot);
                }
                next = next + delta;
            }

            println!("Robot: {:?}", robot);
            println!("Num boxes: {}", boxes.len());
            println!("Boxes: {:?}\n", boxes);
        } else {
            println!("Can't move {:?}\n", instruction);
        }
    }

    println!("Robot: {:?}", robot);
    println!("Boxes: {:?}", boxes);

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
