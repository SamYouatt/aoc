use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

fn part_1(input: &str) -> isize {
    let mut galaxies = vec![];

    let grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(yi, line)| {
            line.chars()
                .enumerate()
                .map(|(xi, char)| {
                    if char == '#' {
                        galaxies.push((xi, yi))
                    };
                    char == '#'
                })
                .collect()
        })
        .collect();

    let empty_rows = (0..grid.len()).filter(|&row| grid[row].iter().all(|is_star| !is_star));
    let empty_cols = (0..grid[0].len()).filter(|&col| (0..grid.len()).all(|row| !grid[row][col]));

    for row in empty_rows.rev() {
        for galaxy in &mut galaxies {
            if galaxy.1 > row {
                galaxy.1 += 1;
            }
        }
    }

    for col in empty_cols.rev() {
        for galaxy in &mut galaxies {
            if galaxy.0 > col {
                galaxy.0 += 1;
            }
        }
    }

    galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| {
            isize::abs(g1.1 as isize - g2.1 as isize) + isize::abs(g1.0 as isize - g2.0 as isize)
        })
        .sum()
}
