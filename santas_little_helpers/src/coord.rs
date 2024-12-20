use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

/// Will cast x and y to isize, will panic if this is bad
/// Example: coord!(3, 2)
#[macro_export]
macro_rules! coord {
    ($x:expr, $y:expr) => {
        crate::coord::Coord {
            x: $x as isize,
            y: $y as isize,
        }
    };
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Compute the manhattan (taxi cab) distance between two points
    pub fn manhattan_dist(&self, other: &Coord) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Delta {
    pub dx: isize,
    pub dy: isize,
}

impl Delta {
    pub fn new(dx: isize, dy: isize) -> Self {
        Self { dx, dy }
    }
}

/// Will cast x and y to isize, will panic if this is bad
/// Example: delta!(-1, 2)
#[macro_export]
macro_rules! delta {
    ($dx:expr, $dy:expr) => {
        crate::coord::Delta {
            dx: $dx as isize,
            dy: $dy as isize,
        }
    };
}

impl Add<Coord> for Delta {
    type Output = Coord;

    fn add(self, coord: Coord) -> Coord {
        Coord {
            x: self.dx + coord.x,
            y: self.dy + coord.y,
        }
    }
}

impl Add<Delta> for Coord {
    type Output = Coord;

    fn add(self, delta: Delta) -> Coord {
        Coord {
            x: self.x + delta.dx,
            y: self.y + delta.dy,
        }
    }
}

impl Sub<Coord> for Delta {
    type Output = Coord;

    fn sub(self, coord: Coord) -> Coord {
        Coord {
            x: self.dx - coord.x,
            y: self.dy - coord.y,
        }
    }
}

impl Sub<Delta> for Coord {
    type Output = Coord;

    fn sub(self, delta: Delta) -> Coord {
        Coord {
            x: self.x - delta.dx,
            y: self.y - delta.dy,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_macro() {
        let coord = coord!(3, 2);
        assert_eq!(coord, Coord::new(3, 2));
    }

    #[test]
    fn coord_macro_casts_usize() {
        let x = 3_usize;
        let y = 2_usize;
        let coord = coord!(x, y);
        assert_eq!(coord, Coord::new(3, 2));
    }
}
