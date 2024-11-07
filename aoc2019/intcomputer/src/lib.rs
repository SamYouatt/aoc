pub mod computer;
pub mod days;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
