use std::collections::HashSet;

use aoc_util::{coordinate::Coordinate, direction::Direction};

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
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

fn part_2(input: &str) -> usize {
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

    // Couple of things to note about the inputs. First the grid is 131x131 and the start is in the
    // center. Also key is that there is a line of empty tiles in each direction from the centre
    // all the way to the edge. This means it takes 65 steps to get from the centre to the edge of
    // the square. To get to the edge of the next tile takes 65 + 131 steps. The second tile 65 +
    // 131 + 131 and so on ...
    //
    // If you look at the number of steps those take they form a quadratic sequence (can probs
    // prove this somehow but I just chose some numbers and the graph looked quadratic).
    //
    // The second thing to note about the input is that the questions step number has the following
    // property: steps % 131 = 65. So its of the same form as the series just seen above. 131 * n +
    // 65.
    //
    // So take those three pieces of the series above, f(65), f(65 + 131), f(65 + 131 + 131). From
    // those we can interpolate the quadratic formula, i.e. deduce a,b,c in y = ax^2 + bx + c
    //
    // From there its just a case of putting in the appropriate value for the sequence. To do this
    // take steps / 131 (floored its not an integer) and pass that into f(x). This is then the
    // answer
    let f1 = positions_in_steps(&grid, 65);
    let f2 = positions_in_steps(&grid, 65 + 131);
    let f3 = positions_in_steps(&grid, 65 + (131 * 2));

    println!("f1: {}", f1);
    println!("f2: {}", f2);
    println!("f3: {}", f3);

    // solved a,b,c using wolfram alpha but could do Lagrange quadratic interpolation or geometric
    // sequence
    let f = |x: usize| (15449 * x * x) + (15541 * x) + 3906;

    let goal_x = 26501365 / 131;
    f(goal_x)
}

fn positions_in_steps(grid: &Vec<Vec<Tile>>, max_steps: usize) -> usize {
    // same as part 1 but add in 'wrapping' to do infinite grid stuff
    let start = get_start(&grid).unwrap();

    let mut stack: Vec<(Coordinate, usize)> = Vec::from_iter([(start, 0)]);

    let mut reached_positions = HashSet::new();
    let mut previously_calculated: HashSet<(Coordinate, usize)> = HashSet::new();

    while let Some((current_pos, steps)) = stack.pop() {
        previously_calculated.insert((current_pos, steps));

        if steps == max_steps {
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
            // wrapping here to simulate infinite grid
            let wrapped_position = get_wrapped_coord(&next_position);

            if is_free(&wrapped_position, &grid)
                && !previously_calculated.contains(&(next_position, steps + 1))
            {
                stack.push((next_position, steps + 1));
            }
        }
    }

    reached_positions.len()
}

fn get_wrapped_coord(coordinate: &Coordinate) -> Coordinate {
    let new_y = coordinate.y.rem_euclid(131);
    let new_x = coordinate.x.rem_euclid(131);

    Coordinate::new(new_x, new_y)
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
