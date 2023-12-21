use std::collections::HashSet;

use aoc_util::{coordinate::Coordinate, direction::Direction};

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

#[derive(PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
    Start,
}

fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Garden,
                    '#' => Tile::Rock,
                    _ => Tile::Start,
                })
                .collect()
        })
        .collect();

    let start = get_start(&grid).unwrap();

    let mut stack: Vec<(Coordinate, usize)> = Vec::from_iter([(start, 0)]);

    let mut reached_positions = HashSet::new();
    let mut previously_calculated: HashSet<(Coordinate, usize)> = HashSet::new();

    while let Some((current_pos, steps)) = stack.pop() {
        previously_calculated.insert((current_pos, steps));

        if steps == 64 {
            reached_positions.insert(current_pos);
            continue;
        }

        for direction in [
            Direction::North,
            Direction::East,
            Direction::West,
            Direction::South,
        ] {
            let next_position = current_pos.move_dir(&direction);

            if !is_valid_coord(&next_position, &grid) {
                continue;
            }

            if is_free(&next_position, &grid)
                && !previously_calculated.contains(&(next_position, steps + 1))
            {
                stack.push((next_position, steps + 1));
            }
        }
    }

    reached_positions.len()
}

fn get_start(grid: &Vec<Vec<Tile>>) -> Option<Coordinate> {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &Tile::Start {
                return Some(Coordinate::new(x as isize, y as isize));
            }
        }
    }

    None
}

fn is_valid_coord(coord: &Coordinate, grid: &Vec<Vec<Tile>>) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    return coord.x >= 0 && coord.y >= 0 && coord.x < cols as isize && coord.y < rows as isize;
}

fn is_free(coord: &Coordinate, grid: &Vec<Vec<Tile>>) -> bool {
    let x = coord.x as usize;
    let y = coord.y as usize;

    match grid[y][x] {
        Tile::Garden => true,
        Tile::Rock => false,
        Tile::Start => true,
    }
}
