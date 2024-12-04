fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut total = 0;
    for (row_i, row) in grid.iter().enumerate() {
        for (col_i, &char) in row.iter().enumerate() {
            if char != 'X' {
                continue;
            }

            // Left
            if col_i.checked_sub(3).is_some() {
                let m = grid[row_i][col_i - 1];
                let a = grid[row_i][col_i - 2];
                let s = grid[row_i][col_i - 3];

                if let ('M', 'A', 'S') = (m, a, s) {
                    total += 1;
                }
            }

            // Up left
            if row_i.checked_sub(3).is_some() && col_i.checked_sub(3).is_some() {
                let m = grid[row_i - 1][col_i - 1];
                let a = grid[row_i - 2][col_i - 2];
                let s = grid[row_i - 3][col_i - 3];

                if let ('M', 'A', 'S') = (m, a, s) {
                    total += 1;
                }
            }

            // Up
            if row_i.checked_sub(3).is_some() {
                let m = grid[row_i - 1][col_i];
                let a = grid[row_i - 2][col_i];
                let s = grid[row_i - 3][col_i];

                if let ('M', 'A', 'S') = (m, a, s) {
                    total += 1;
                }
            }

            // Up right
            if row_i.checked_sub(3).is_some() && grid[row_i].get(col_i + 3).is_some() {
                let m = grid[row_i - 1][col_i + 1];
                let a = grid[row_i - 2][col_i + 2];
                let s = grid[row_i - 3][col_i + 3];

                if let ('M', 'A', 'S') = (m, a, s) {
                    total += 1;
                }
            }

            // Right
            if grid[row_i].get(col_i + 3).is_some() {
                let m = grid[row_i][col_i + 1];
                let a = grid[row_i][col_i + 2];
                let s = grid[row_i][col_i + 3];

                if let ('M', 'A', 'S') = (m, a, s) {
                    total += 1;
                }
            }

            // Down right
            if grid.get(row_i + 3).is_some() && grid[row_i].get(col_i + 3).is_some() {
                let m = grid[row_i + 1][col_i + 1];
                let a = grid[row_i + 2][col_i + 2];
                let s = grid[row_i + 3][col_i + 3];

                if let ('M', 'A', 'S') = (m, a, s) {
                    total += 1;
                }
            }

            // Down
            if grid.get(row_i + 3).is_some() {
                let m = grid[row_i + 1][col_i];
                let a = grid[row_i + 2][col_i];
                let s = grid[row_i + 3][col_i];

                if let ('M', 'A', 'S') = (m, a, s) {
                    total += 1;
                }
            }

            // Down left
            if grid.get(row_i + 3).is_some() && col_i.checked_sub(3).is_some() {
                let m = grid[row_i + 1][col_i - 1];
                let a = grid[row_i + 2][col_i - 2];
                let s = grid[row_i + 3][col_i - 3];

                if let ('M', 'A', 'S') = (m, a, s) {
                    total += 1;
                }
            }
        }
    }

    total
}

fn part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut total = 0;
    for (row_i, row) in grid.iter().enumerate() {
        for (col_i, &char) in row.iter().enumerate() {
            if char != 'A' {
                continue;
            }

            if !row_i.checked_sub(1).is_some()
                || !col_i.checked_sub(1).is_some()
                || !grid.get(row_i + 1).is_some()
                || !row.get(col_i + 1).is_some()
            {
                continue;
            }

            match (
                grid[row_i - 1][col_i - 1],
                grid[row_i - 1][col_i + 1],
                grid[row_i + 1][col_i + 1],
                grid[row_i + 1][col_i - 1],
            ) {
                ('M', 'M', 'S', 'S')
                | ('S', 'M', 'M', 'S')
                | ('S', 'S', 'M', 'M')
                | ('M', 'S', 'S', 'M') => total += 1,
                _ => continue,
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(part_1(input), 18);
    }

    #[test]
    fn test_part_2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        assert_eq!(part_2(input), 9);
    }
}
