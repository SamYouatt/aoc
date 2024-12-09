use crate::{coord::Coord, delta, directions::Direction};

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
    pub fn neighbours(&self, current: Coord) -> Vec<Coord> {
        let mut neighbours = Vec::new();
        for delta in [delta!(-1, 0), delta!(1, 0), delta!(0, -1), delta!(0, 1)].iter() {
            let applied = current.apply_delta(&delta);
            if self.in_bounds(&applied) {
                neighbours.push(applied);
            }
        }

        neighbours
    }

    // TODO: make this an iterator
    /// Like neighbours but only the ones with the same value as the current
    pub fn matching_neighbours(&self, current: Coord) -> Vec<Coord> {
        let neighbours = self.neighbours(current);
        let current_val = self.get(&current);
        neighbours
            .iter()
            .filter(|&x| self.get(x) == current_val)
            .map(|x| x.to_owned())
            .collect()
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
