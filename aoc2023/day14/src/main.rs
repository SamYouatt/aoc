use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

fn part_1(input: &str) -> usize {
    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    tilt_north(&mut grid);

    count_load(&grid)
}

fn part_2(input: &str) -> usize {
    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut seen: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let num_cycles = 1000000000;

    for cycle in 1..num_cycles {
        for _ in 0..4 {
            tilt_north(&mut grid);
            grid = rotate_left(&grid);
        }

        if let Some(previous_seen) = seen.get(&grid) {
            let remaining = num_cycles - cycle;
            // how often does this layout come around
            let period = cycle - previous_seen;

            // if its a divisor then we know by the end we are gonna be the same as we are now
            if remaining % period == 0 {
                break;
            }
        }

        seen.insert(grid.clone(), cycle);
    }

    count_load(&grid)
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for row in 1..grid.len() {
        for col in 0..grid[0].len() {
            let mut pointer = row;

            while pointer > 0 {
                if grid[pointer - 1][col] == '.' && grid[pointer][col] == 'O' {
                    grid[pointer][col] = '.';
                    grid[pointer - 1][col] = 'O';

                    pointer -= 1;
                } else {
                    break;
                }
            }
        }
    }
}

fn count_load(grid: &Vec<Vec<char>>) -> usize {
    (0..grid.len())
        .map(|row| {
            (0..grid[0].len())
                .filter(|&col| grid[row][col] == 'O')
                .map(|_col| grid.len() - row)
                .sum::<usize>()
        })
        .sum()
}

fn rotate_left(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut rotated_grid = vec![vec!['X'; num_cols]; num_rows];

    for row in 0..num_rows {
        for col in 0..num_cols {
            let new_row = col;
            let new_col = num_rows - row - 1;

            rotated_grid[new_row][new_col] = grid[row][col];
        }
    }

    rotated_grid
}

#[test]
fn test_tilt_north() {
    let original = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    let mut original: Vec<Vec<_>> = original
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let after = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";

    let after: Vec<Vec<_>> = after.lines().map(|line| line.chars().collect()).collect();

    tilt_north(&mut original);

    assert_eq!(original, after);
}
