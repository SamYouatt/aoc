use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use aoc_util::{coordinate::Coordinate, direction::Direction};
use hashbrown::HashSet;

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
                    '#' => Tile::Tree,
                    _ => Tile::Path,
                })
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut graph = HashMap::<_, Vec<_>>::new();

    // build the initial graph
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let coordinate = Coordinate::new(x as isize, y as isize);

            let neighbours: Vec<Coordinate> = match tile {
                Tile::Tree => continue,
                _ => [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .iter()
                .map(|dir| coordinate.move_dir(dir))
                .collect(),
            };

            let node = graph.entry(coordinate).or_default();

            for neighbour in neighbours {
                if neighbour.x < 0
                    || neighbour.x >= cols as isize
                    || neighbour.y < 0
                    || neighbour.y >= rows as isize
                {
                    continue;
                }

                if grid[neighbour.y as usize][neighbour.x as usize] == Tile::Tree {
                    continue;
                }

                node.push((neighbour, 1));
            }
        }
    }

    let collapsible_coords: Vec<_> = graph
        .iter()
        .filter(|(_, edges)| edges.len() == 2)
        .map(|(&coord, _cost)| coord)
        .collect();

    // collapse A-n-B-m-C into A-n+m-C
    for coord_b in collapsible_coords {
        // remove the middle node but grab its connections
        let node_b_connections = graph.remove(&coord_b).unwrap();

        // grab the coordinate and cost of those connections
        let (coord_a, cost_a) = node_b_connections[0];
        let (coord_c, cost_c) = node_b_connections[1];

        // for each of the connecting nodes, point them to each other instead of b and update the
        // cost as the sum of the costs
        let node_a = graph.get_mut(&coord_a).unwrap();
        let b_index = node_a
            .iter()
            .position(|&connection| connection.0 == coord_b)
            .unwrap();
        node_a[b_index] = (coord_c, cost_a + cost_c);

        let node_c = graph.get_mut(&coord_c).unwrap();
        let b_index = node_c
            .iter()
            .position(|&connection| connection.0 == coord_b)
            .unwrap();
        node_c[b_index] = (coord_a, cost_a + cost_c);
    }

    let final_row = grid.len() - 1;

    find_longest_graph_path(
        &graph,
        Coordinate::new(1, 0),
        &mut HashSet::new(),
        final_row,
    )
    .unwrap()
}

fn find_longest_graph_path(
    graph: &HashMap<Coordinate, Vec<(Coordinate, usize)>>,
    current_pos: Coordinate,
    previous: &mut HashSet<Coordinate>,
    final_row: usize,
) -> Option<usize> {
    if current_pos.y == final_row as isize {
        return Some(0);
    }

    let mut longest_path = None;

    for &(connection, cost) in &graph[&current_pos] {
        if previous.contains(&connection) {
            continue;
        }

        previous.insert(connection);

        if let Some(path_length) = find_longest_graph_path(graph, connection, previous, final_row) {
            longest_path = Some(longest_path.unwrap_or(0).max(path_length + cost))
        }

        previous.remove(&connection);
    }

    longest_path
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
