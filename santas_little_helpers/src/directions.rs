use crate::{coord::Delta, delta};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn deltas() -> Vec<Delta> {
        [delta!(-1, 0), delta!(1, 0), delta!(0, -1), delta!(0, 1)].to_vec()
    }
}
