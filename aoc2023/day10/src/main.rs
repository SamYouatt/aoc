use std::collections::HashSet;

type Grid = Vec<Vec<char>>;
type Coord = (usize, usize);

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

fn part_1(input: &str) -> usize {
    let grid = parse_grid(input);
    let start = get_start(&grid);
    let pipe_loop = get_loop(&grid, start);

    pipe_loop.len() / 2
}

fn part_2(input: &str) -> usize {
    let grid = parse_grid(input);
    let start = get_start(&grid);
    let pipe_loop = get_loop(&grid, start);

    let pipe_set: HashSet<Coord> = HashSet::from_iter(pipe_loop.iter().cloned());

    let mut junked_grid: Grid = grid
        .iter()
        .enumerate()
        .map(|(yi, row)| {
            row.iter()
                .enumerate()
                .map(|(xi, tile)| {
                    if !pipe_set.contains(&(xi, yi)) {
                        return '.';
                    }
                    tile.clone()
                })
                .collect()
        })
        .collect();

    junked_grid[start.1][start.0] = start_pipe(&pipe_loop, start);

    let mut enclosed = 0;

    // start at a tile and look leftwards to the edge
    // as you go count the number of north and south directions of the pipes along the way
    // take the minumum of those and check if its odd
    // if it is odd then the tile is enclosed
    // Source: this is the crossing number algorithm used to find points inside a polygon
    for (yi, row) in junked_grid.iter().enumerate() {
        for (xi, &tile) in row.iter().enumerate() {
            if tile != '.' {
                continue;
            }

            let (north, south) = count_n_and_s(&row, (xi, yi));

            if north.min(south) % 2 != 0 {
                enclosed += 1;
            }
        }
    }

    enclosed
}

fn count_n_and_s(row: &Vec<char>, start: Coord) -> (usize, usize) {
    let mut north = 0;
    let mut south = 0;

    let mut current = start;

    while current.0 < row.len() {
        match row[current.0] {
            '|' => {
                north += 1;
                south += 1;
            }
            'L' | 'J' => {
                north += 1;
            }
            'F' | '7' => {
                south += 1;
            }
            _ => {}
        }

        current = (current.0.wrapping_sub(1), current.1);
    }

    (north, south)
}

fn start_pipe(pipes: &Vec<Coord>, start: Coord) -> char {
    let next = pipes[1];
    let prev = pipes.last().unwrap();

    let to_next = (
        next.0 as isize - start.0 as isize,
        next.1 as isize - start.1 as isize,
    );
    let to_prev = (
        prev.0 as isize - start.0 as isize,
        prev.1 as isize - start.1 as isize,
    );

    match (to_next, to_prev) {
        ((0, 1), (0, -1)) | ((0, -1), (0, 1)) => '|',
        ((1, 0), (-1, 0)) | ((-1, 0), (1, 0)) => '-',
        ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => 'F',
        ((0, -1), (-1, 0)) | ((-1, 0), (0, -1)) => 'J',
        ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => '7',
        ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => 'L',
        _ => panic!("Something wrong"),
    }
}

fn parse_grid(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_start(grid: &Grid) -> Coord {
    for (yi, y) in grid.iter().enumerate() {
        for (xi, x) in y.iter().enumerate() {
            if x == &'S' {
                return (xi, yi);
            }
        }
    }
    panic!("Start not found");
}

fn get_loop(grid: &Grid, start: Coord) -> Vec<Coord> {
    let adjacents = vec![
        (start.0, start.1.wrapping_sub(1)),
        (start.0 + 1, start.1),
        (start.0, start.1 + 1),
        (start.0.wrapping_sub(1), start.1),
    ];

    let first_pipe = adjacents
        .iter()
        .find(|&&coord| {
            if let Some((first, second)) = next(&grid, coord) {
                return first == start || second == start;
            }
            false
        })
        .copied()
        .expect("Couldn't find next step from start");

    let mut visited_pipes: Vec<Coord> = vec![start];
    let mut current = first_pipe;

    while grid[current.1][current.0] != 'S' {
        visited_pipes.push(current);

        let (first, second) = next(&grid, current).expect("Expected to be on loop now");

        // Don't want to go back on ourselves immediately
        if visited_pipes[visited_pipes.len() - 2] == first {
            current = second;
        } else {
            current = first;
        }
    }

    visited_pipes
}

fn next(grid: &Grid, (x, y): Coord) -> Option<(Coord, Coord)> {
    if y >= grid.len() || x >= grid[0].len() {
        return None;
    }

    let pipe = grid[y][x];

    match pipe {
        '|' => Some(((x, y.wrapping_sub(1)), (x, y + 1))),
        '-' => Some(((x.wrapping_sub(1), y), (x + 1, y))),
        'L' => Some(((x, y.wrapping_sub(1)), (x + 1, y))),
        'J' => Some(((x.wrapping_sub(1), y), (x, y.wrapping_sub(1)))),
        '7' => Some(((x.wrapping_sub(1), y), (x, y + 1))),
        'F' => Some(((x, y + 1), (x + 1, y))),
        '.' | 'S' | _ => None,
    }
}
