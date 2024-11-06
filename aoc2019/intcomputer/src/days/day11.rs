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

struct Robot<'a> {
    position: Point,
    facing: Direction,
    sender: Sender<i64>,
    receiver: Receiver<i64>,
    grid: &'a mut HashMap<Point, Colour>,
}

impl<'a> Robot<'a> {
    pub fn new(
        sender: Sender<i64>,
        receiver: Receiver<i64>,
        grid: &'a mut HashMap<Point, Colour>,
    ) -> Self {
        Self {
            position: Point::new(0, 0),
            facing: Direction::Up,
            sender,
            receiver,
            grid,
        }
    }

    pub fn run(&mut self) {
        // Tell computer about colour under wheel: 0 for black, 1 for white
        // Listen for instruction from computer
        // First colour: 0 = black, 1 = white
        // Second direction: 0 = left 90, 1 = right 90
        // Then move forwards once

        self.sender
            .send(self.colour_message())
            .expect("computer should not close receiver");
        // Flag to control current action - painting or moving
        let mut painting = true;
        while let Ok(instruction) = self.receiver.recv() {
            match painting {
                true => {
                    let colour = match instruction {
                        0 => Colour::Black,
                        1 => Colour::White,
                        _ => panic!("unexpected instruction from computer"),
                    };
                    self.grid.insert(self.position, colour);
                    painting = false;
                }
                false => {
                    // 0 = anti-clockwise, 1 = clockwise
                    self.facing = match instruction {
                        0 => rotate(&self.facing, false),
                        1 => rotate(&self.facing, true),
                        _ => panic!("unexpected instruction from computer"),
                    };

                    self.position = self.position.forwards(&self.facing);

                    // Computer may have halted and closed its receiver but we don't care
                    let _ = self.sender.send(self.colour_message());
                    painting = true;
                }
            }
        }
    }

    fn colour_message(&self) -> i64 {
        match self.grid.get(&self.position) {
            Some(Colour::White) => 1,
            Some(Colour::Black) | None => 0,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let tape = parse_tape(input);
    let (computer_sender, computer_receiver) = mpsc::channel();
    let (robot_sender, robot_receiver) = mpsc::channel();

    let mut grid: HashMap<Point, Colour> = HashMap::new();
    let mut computer = Computer::load(&tape, computer_receiver, robot_sender);
    let mut robot = Robot::new(computer_sender, robot_receiver, &mut grid);

    // Hoping this will close the robot's sender and so it will know to stop
    thread::spawn(move || {
        computer.run();
        drop(computer);
    });

    robot.run();

    let visited: HashSet<Point> = grid.keys().map(|p| *p).collect();

    visited.len()
}
