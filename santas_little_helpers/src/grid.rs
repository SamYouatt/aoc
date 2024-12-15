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

    // TODO: make this an iterator
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

    // TODO: make this an iterator
    /// Like neighbours but only the ones with the same value as the current
    pub fn matching_neighbours<'a>(&'a self, current: Coord) -> impl Iterator<Item = Coord> + 'a {
        let current_val = self.get(&current);
        self.neighbours(current)
            .filter(move |x| self.get(&x) == current_val)
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
