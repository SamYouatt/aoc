use crate::coord::Coord;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        todo!()
    }

    /// Get at coordinate, will panic if out of bounds
    pub fn get(&self, coord: &Coord) -> &T {
        &self.grid[coord.y as usize][coord.x as usize]
    }

    /// Will return none if out of bounds
    pub fn try_get(&self, coord: &Coord) -> Option<&T> {
        let y: usize = coord.y.try_into().ok()?;
        let x: usize = coord.x.try_into().ok()?;
        self.grid.get(y)?.get(x)
    }

    pub fn in_bounds(&self, coord: &Coord) -> bool {
        coord.x >= 0
            && coord.y >= 0
            && coord.x < self.width as isize
            && coord.y < self.height as isize
    }
}
