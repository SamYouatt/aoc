use std::{
    collections::{HashMap, HashSet},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::{parse_tape, Computer};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn forwards(self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
enum Colour {
    White,
    Black,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn rotate(current: &Direction, clockwise: bool) -> Direction {
    match (current, clockwise) {
        (Direction::Up, true) | (Direction::Down, false) => Direction::Right,
        (Direction::Up, false) | (Direction::Down, true) => Direction::Left,
        (Direction::Left, true) | (Direction::Right, false) => Direction::Up,
        (Direction::Left, false) | (Direction::Right, true) => Direction::Down,
    }
}

fn colour_message(position: &Point, grid: &HashMap<Point, Colour>) -> i64 {
    match grid.get(position) {
        Some(Colour::White) => 1,
        Some(Colour::Black) | None => 0,
    }
}

fn robot(sender: Sender<i64>, receiver: Receiver<i64>, grid: &mut HashMap<Point, Colour>) {
    let mut position = Point::new(0, 0);
    let mut facing = Direction::Up;
    let mut painting = true;

    sender
        .send(colour_message(&position, &grid))
        .expect("computer should not close receiver");

    while let Ok(instruction) = receiver.recv() {
        match painting {
            true => {
                let colour = match instruction {
                    0 => Colour::Black,
                    1 => Colour::White,
                    _ => panic!("unexpected instruction from computer"),
                };
                grid.insert(position, colour);
                painting = false;
            }
            false => {
                // 0 = anti-clockwise, 1 = clockwise
                facing = match instruction {
                    0 => rotate(&facing, false),
                    1 => rotate(&facing, true),
                    _ => panic!("unexpected instruction from computer"),
                };

                position = position.forwards(&facing);

                // Computer may have halted and closed its receiver but we don't care
                let _ = sender.send(colour_message(&position, &grid));
                painting = true;
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let tape = parse_tape(input);
    let (computer_sender, computer_receiver) = mpsc::channel();
    let (robot_sender, robot_receiver) = mpsc::channel();

    let mut grid: HashMap<Point, Colour> = HashMap::new();
    let mut computer = Computer::load(&tape, computer_receiver, robot_sender);

    // Hoping this will close the robot's sender and so it will know to stop
    thread::spawn(move || {
        computer.run();
        drop(computer);
    });

    robot(computer_sender, robot_receiver, &mut grid);

    let visited: HashSet<Point> = grid.keys().map(|p| *p).collect();

    visited.len()
}
