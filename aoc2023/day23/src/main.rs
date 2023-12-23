use std::collections::HashSet;

use aoc_util::{coordinate::Coordinate, direction::Direction};

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
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

    let mut path_lengths: Vec<usize> = vec![];
    find_longest_path(&grid, start, HashSet::new(), 0, &mut path_lengths);

    *path_lengths.iter().max().unwrap()
}

fn part_2(input: &str) -> usize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' | '^' | '>' | 'v' | '<' => Tile::Path,
                    '#' => Tile::Tree,
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

    let mut path_lengths: Vec<usize> = vec![];
    find_longest_path(&grid, start, HashSet::new(), 0, &mut path_lengths);

    *path_lengths.iter().max().unwrap()
}

fn find_longest_path(
    grid: &Vec<Vec<Tile>>,
    current_pos: Coordinate,
    previous: HashSet<Coordinate>,
    path_length: usize,
    path_lengths: &mut Vec<usize>,
) {
    let rows = grid.len();
    let cols = grid[0].len();

    if current_pos.y as usize == grid.len() - 1 {
        println!("New path length found: {}", path_length);
        path_lengths.push(path_length);
        return;
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
                new_previous.insert(current_pos);

                find_longest_path(&grid, next_pos, new_previous, path_length + 1, path_lengths);
            }
        }
        Tile::Slope(slope) => {
            let next_pos = current_pos.move_dir(&slope);

            if next_pos.x < 0
                || next_pos.x >= cols as isize
                || next_pos.y < 0
                || next_pos.y >= rows as isize
            {
                return;
            }

            if previous.contains(&next_pos) {
                return;
            }

            let mut new_previous = previous.clone();
            new_previous.insert(current_pos);

            find_longest_path(&grid, next_pos, new_previous, path_length + 1, path_lengths);
        }
        _ => panic!("Shouldn't be able to be on a tree"),
    }
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

#[test]
fn part_2_example() {
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

    let answer = part_2(input);

    assert_eq!(answer, 154);
}
