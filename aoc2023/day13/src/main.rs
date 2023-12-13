fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
}

fn part_1(input: &str) -> usize {
    let patterns: Vec<Vec<String>> = input
        .split("\n\n")
        .map(|pattern| pattern.lines().map(|line| line.chars().collect()).collect())
        .collect();

    let horizontal_points: Vec<_> = patterns
        .iter()
        .flat_map(|pattern| find_reflection(pattern))
        .collect();

    let transposed: Vec<Vec<_>> = patterns.iter().map(|pattern| transpose(pattern)).collect();

    let vertical_points: Vec<_> = transposed
        .iter()
        .flat_map(|pattern| find_reflection(pattern))
        .collect();

    vertical_points.iter().sum::<usize>() + 100 * horizontal_points.iter().sum::<usize>()
}

fn find_reflection(pattern: &Vec<String>) -> Option<usize> {
    'ref_loop: for ref_point in 0..(pattern.len() - 1) {
        let mut top_pointer = ref_point;
        let mut bottom_pointer = ref_point + 1;

        loop {
            if pattern[top_pointer] != pattern[bottom_pointer] {
                continue 'ref_loop;
            }

            if top_pointer == 0 || bottom_pointer == (pattern.len() - 1) {
                return Some(ref_point + 1);
            }

            top_pointer -= 1;
            bottom_pointer += 1;
        }
    }

    None
}

fn transpose(slices: &Vec<String>) -> Vec<String> {
    if slices.is_empty() {
        return Vec::new();
    }

    let cols = slices.iter().map(|s| s.len()).max().unwrap_or(0);

    (0..cols)
        .map(|col| {
            slices
                .iter()
                .map(|s| s.chars().nth(col).unwrap_or(' '))
                .collect()
        })
        .collect()
}

#[test]
fn test_find_horizontal_reflection() {
    let pattern = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    let pattern = pattern.lines().map(|line| line.chars().collect()).collect();

    assert_eq!(find_reflection(&pattern), Some(4));
}

#[test]
fn test_no_reflection() {
    let pattern = "#....
.#...
..#..
...#.";

    let pattern = pattern.lines().map(|line| line.chars().collect()).collect();

    assert_eq!(find_reflection(&pattern), None);
}

#[test]
fn test_transpose() {
    let pattern = "####
....
####";

    let pattern = pattern.lines().map(|line| line.chars().collect()).collect();

    let transposed = transpose(&pattern);

    println!("{:#?}", transposed);
}
