use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

fn part_1(input: &str) -> isize {
    let (grid, mut galaxies) = parse_grid(input);

    expand_coords(&mut galaxies, &grid, 2);

    sum_mc_distances(&galaxies)
}

fn part_2(input: &str) -> isize {
    let (grid, mut galaxies) = parse_grid(input);

    expand_coords(&mut galaxies, &grid, 1_000_000);

    sum_mc_distances(&galaxies)
}

fn parse_grid(input: &str) -> (Vec<Vec<bool>>, Vec<(usize, usize)>) {
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

    (grid, galaxies)
}

fn expand_coords(
    galaxies: &mut Vec<(usize, usize)>,
    grid: &Vec<Vec<bool>>,
    expansion_factor: usize,
) {
    let empty_rows: Vec<_> = (0..grid.len())
        .filter(|&row| grid[row].iter().all(|is_star| !is_star))
        .collect();

    let empty_cols: Vec<_> = (0..grid[0].len())
        .filter(|&col| (0..grid.len()).all(|row| !grid[row][col]))
        .collect();

    for galaxy in &mut *galaxies {
        for &row in empty_rows.iter().rev() {
            if galaxy.1 > row {
                galaxy.1 += expansion_factor - 1;
            }
        }

        for &col in empty_cols.iter().rev() {
            if galaxy.0 > col {
                galaxy.0 += expansion_factor - 1;
            }
        }
    }
}

fn sum_mc_distances(galaxies: &Vec<(usize, usize)>) -> isize {
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| {
            isize::abs(g1.1 as isize - g2.1 as isize) + isize::abs(g1.0 as isize - g2.0 as isize)
        })
        .sum()
}
