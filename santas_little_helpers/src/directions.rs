use crate::{coord::Delta, delta};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> Delta {
        match self {
            Direction::Up => delta!(0, -1),
            Direction::Down => delta!(0, 1),
            Direction::Left => delta!(-1, 0),
            Direction::Right => delta!(1, 0),
        }
    }

    pub fn iterator() -> impl Iterator<Item = Direction> {
        [Direction::Up, Direction::Right, Direction::Down, Direction::Left].into_iter()
    }

    pub fn deltas() -> impl Iterator<Item = Delta> {
        [delta!(-1, 0), delta!(1, 0), delta!(0, -1), delta!(0, 1)].into_iter()
    }

    pub fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}
