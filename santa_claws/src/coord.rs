#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: isize,
    pub y: isize,
}

/// Will cast x and y to isize, will panic if this is bad
/// Example: coord!(3, 2)
macro_rules! coord {
    ($x:expr, $y:expr) => {
        Coord {
            x: $x as isize,
            y: $y as isize,
        }
    };
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
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
