type Grid = Vec<Vec<char>>;
type Coord = (usize, usize);

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

fn part_1(input: &str) -> usize {
    let grid = parse_grid(input);
    let start = get_start(&grid);

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

    visited_pipes.len() / 2
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
