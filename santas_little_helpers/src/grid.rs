use crate::{coord::Coord, directions::Direction};

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<T>>,
}

impl<T: PartialEq> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    pub fn init(init_tile: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        let mut rows = Vec::new();
        for _ in 0..height {
            let row = vec![init_tile.clone(); width];
            rows.push(row);
        }

        Self {
            grid: rows,
            width,
            height,
        }
    }

    pub fn from_vecs(grid: Vec<Vec<T>>) -> Self {
        Self {
            width: grid[0].len(),
            height: grid.len(),
            grid,
        }
    }

    /// Get at coordinate, will panic if out of bounds
    pub fn get(&self, coord: &Coord) -> &T {
        &self.grid[coord.y as usize][coord.x as usize]
    }

    pub fn get_mut(&mut self, coord: Coord) -> &mut T {
        &mut self.grid[coord.y as usize][coord.x as usize]
    }

    pub fn set(&mut self, coord: Coord, tile: T) {
        self.grid[coord.y as usize][coord.x as usize] = tile;
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

    /// An iterator over all neighbours that are in bounds. No checks about the tile are made
    pub fn neighbours<'a>(&'a self, current: Coord) -> impl Iterator<Item = Coord> + 'a {
        let mut neighbours = Vec::new();
        for delta in Direction::deltas() {
            let applied = current + delta;
            if self.in_bounds(&applied) {
                neighbours.push(applied);
            }
        }

        neighbours.into_iter()
    }

    /// An iterator over all in bound neibhbours that match the required tile value
    pub fn matching_neighbours<'a>(&'a self, current: Coord, tile: T) -> impl Iterator<Item = Coord> + 'a {
        self.neighbours(current)
            .filter(move |x| self.get(&x) == &tile)
    }

    pub fn move_direction(&self, current: &Coord, direction: &Direction) -> Option<Coord> {
        todo!()
    }

    pub fn coord_iter() {
        todo!()
    }
}

impl<T: PartialEq> From<Vec<Vec<T>>> for Grid<T> {
    fn from(vecs: Vec<Vec<T>>) -> Self {
        Grid::from_vecs(vecs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_move_direction_in_bounds() {
        todo!()
    }

    #[test]
    fn grid_move_out_of_bounds() {
        todo!()
    }
}
