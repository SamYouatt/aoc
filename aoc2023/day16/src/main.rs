use std::collections::{HashMap, HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.as_bytes().into()).collect();

    bounce_light(&grid)
}

fn bounce_light(grid: &Vec<Vec<u8>>) -> usize {
    let mut stack: VecDeque<(Coord, Direction)> = VecDeque::with_capacity(20_000);
    let mut seen: HashSet<(Coord, Direction)> = HashSet::new();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut energised: Vec<Vec<bool>> = vec![vec![false; cols]; rows];

    stack.push_back((Coord { x: 0, y: 0 }, Direction::East));

    while let Some((position, direction)) = stack.pop_front() {
        if (position.y < 0
            || position.y >= rows as isize
            || position.x < 0
            || position.x >= cols as isize)
            || !seen.insert((position, direction))
        {
            continue;
        }

        let x = position.x as usize;
        let y = position.y as usize;

        energised[y][x] = true;

        match grid[y][x] {
            b'.' => stack.push_back((point_in_direction(x, y, direction), direction)),
            b'/' => {
                match direction {
                    Direction::North => stack
                        .push_back((point_in_direction(x, y, Direction::East), Direction::East)),
                    Direction::East => stack
                        .push_back((point_in_direction(x, y, Direction::North), Direction::North)),
                    Direction::South => stack
                        .push_back((point_in_direction(x, y, Direction::West), Direction::West)),
                    Direction::West => stack
                        .push_back((point_in_direction(x, y, Direction::South), Direction::South)),
                }
            }
            b'\\' => {
                match direction {
                    Direction::North => stack
                        .push_back((point_in_direction(x, y, Direction::West), Direction::West)),
                    Direction::East => stack
                        .push_back((point_in_direction(x, y, Direction::South), Direction::South)),
                    Direction::South => stack
                        .push_back((point_in_direction(x, y, Direction::East), Direction::East)),
                    Direction::West => stack
                        .push_back((point_in_direction(x, y, Direction::North), Direction::North)),
                }
            }
            b'-' => match direction {
                Direction::East | Direction::West => {
                    stack.push_back((point_in_direction(x, y, direction), direction))
                }
                Direction::North | Direction::South => {
                    stack.push_back((point_in_direction(x, y, Direction::East), Direction::East));
                    stack.push_back((point_in_direction(x, y, Direction::West), Direction::West));
                }
            },
            b'|' => match direction {
                Direction::North | Direction::South => {
                    stack.push_back((point_in_direction(x, y, direction), direction))
                }
                Direction::East | Direction::West => {
                    stack.push_back((point_in_direction(x, y, Direction::North), Direction::North));
                    stack.push_back((point_in_direction(x, y, Direction::South), Direction::South));
                }
            },
            _ => panic!("Unkown grid symbol"),
        }
    }

    energised
        .iter()
        .map(|row| row.iter().filter(|&energised| *energised).count())
        .sum()
}

fn point_in_direction(x: usize, y: usize, direction: Direction) -> Coord {
    match direction {
        Direction::North => Coord {
            x: x as isize,
            y: y as isize - 1,
        },
        Direction::East => Coord {
            x: x as isize + 1,
            y: y as isize,
        },
        Direction::South => Coord {
            x: x as isize,
            y: y as isize + 1,
        },
        Direction::West => Coord {
            x: x as isize - 1,
            y: y as isize,
        },
    }
}

#[test]
fn part_1_test() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    let energised = part_1(input);

    assert_eq!(energised, 46);
}
