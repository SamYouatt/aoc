use std::collections::{HashSet, VecDeque};

use santas_little_helpers::coord;
use santas_little_helpers::coord::Coord;
use santas_little_helpers::directions::Direction;
use santas_little_helpers::grid::Grid;

fn main() {
    let input = include_str!("input.txt");
    let (p1, p2) = both_parts(input);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn both_parts(input: &str) -> (usize, usize) {
    let map: Grid<char> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>()
        .into();

    let mut visited: HashSet<Coord> = HashSet::new();

    let mut part_1_cost = 0;
    let mut part_2_cost = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = coord!(x, y);
            if !visited.contains(&pos) {
                let (area, perim, region) = build_region(pos, &map, &mut visited);
                part_1_cost += area * perim;
                part_2_cost += area * get_sides(&region);
            }
        }
    }

    (part_1_cost, part_2_cost)
}

fn build_region(
    start: Coord,
    map: &Grid<char>,
    visited: &mut HashSet<Coord>,
) -> (usize, usize, HashSet<Coord>) {
    let mut work = VecDeque::new();
    let mut region = HashSet::new();
    let mut perimeter = 0;

    work.push_back(start);
    visited.insert(start);

    while let Some(next_loc) = work.pop_front() {
        region.insert(next_loc);

        let neighbours = map.matching_neighbours(next_loc);
        perimeter += 4 - neighbours.len();

        for neighbour in neighbours.into_iter() {
            if !visited.contains(&neighbour) {
                visited.insert(neighbour);
                work.push_back(neighbour);
            }
        }
    }

    (region.len(), perimeter, region)
}

fn get_sides(area: &HashSet<Coord>) -> usize {
    let mut side_count = 0;

    for direction in Direction::deltas().iter() {
        let mut external_pos = HashSet::new();
        for pos in area {
            let applied = pos.apply_delta(direction);
            if !area.contains(&applied) {
                external_pos.insert(applied);
            }
        }

        let mut redundant_positions = HashSet::new();
        for ext_pos in &external_pos {
            let mut next_on_edge = coord!(ext_pos.x + direction.dy, ext_pos.y + direction.dx);
            while external_pos.contains(&next_on_edge) {
                redundant_positions.insert(next_on_edge);
                next_on_edge = coord!(next_on_edge.x + direction.dy, next_on_edge.y + direction.dx);
            }
        }

        side_count += external_pos.len() - redundant_positions.len();
    }

    side_count
}
