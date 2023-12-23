use std::collections::HashSet;

use aoc_util::{coordinate::Coordinate, direction::Direction};

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Path,
    Tree,
    Slope(Direction),
}

fn part_1(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Tile::Path,
                    '#' => Tile::Tree,
                    '^' => Tile::Slope(Direction::North),
                    '>' => Tile::Slope(Direction::East),
                    'v' => Tile::Slope(Direction::South),
                    '<' => Tile::Slope(Direction::West),
                    _ => panic!("Unknown tile"),
                })
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let start = Coordinate::new(
        grid[0].iter().position(|&tile| tile == Tile::Path).unwrap() as isize,
        0 as isize,
    );
    let end = Coordinate::new(
        grid[rows - 1]
            .iter()
            .position(|&tile| tile == Tile::Path)
            .unwrap() as isize,
        (grid.len() - 1) as isize,
    );

    let mut longest_path = 0;

    let mut stack: Vec<(Coordinate, HashSet<Coordinate>, usize)> =
        Vec::from_iter([(start, HashSet::new(), 0)]);

    while let Some((current_pos, previous, path_length)) = stack.pop() {
        if current_pos == end {
            if path_length > longest_path {
                longest_path = path_length;
            }
        }

        let tile = grid[current_pos.y as usize][current_pos.x as usize];

        match tile {
            Tile::Path => {
                for direction in [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ] {
                    let next_pos = current_pos.move_dir(&direction);

                    if next_pos.x < 0
                        || next_pos.x >= cols as isize
                        || next_pos.y < 0
                        || next_pos.y >= rows as isize
                    {
                        continue;
                    }

                    if previous.contains(&next_pos) {
                        continue;
                    }

                    let next_tile = grid[next_pos.y as usize][next_pos.x as usize];

                    if next_tile == Tile::Tree {
                        continue;
                    }

                    let mut new_previous = previous.clone();
                    new_previous.insert(next_pos);

                    stack.push((next_pos, new_previous, path_length + 1));
                }
            }
            Tile::Slope(slope) => {
                let next_pos = current_pos.move_dir(&slope);

                if next_pos.x < 0
                    || next_pos.x >= cols as isize
                    || next_pos.y < 0
                    || next_pos.y >= rows as isize
                {
                    continue;
                }

                if previous.contains(&next_pos) {
                    continue;
                }

                let mut new_previous = previous.clone();
                new_previous.insert(next_pos);

                stack.push((next_pos, new_previous, path_length + 1));
            }
            _ => panic!("Shouldn't be able to be on a tree"),
        }
    }

    longest_path
}

#[test]
fn part_1_example() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

    let answer = part_1(input);

    assert_eq!(answer, 94);
}
