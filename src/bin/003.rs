use std::{fs, time::Instant};

fn main() {
    let input = fs::read_to_string("/home/sam/Documents/code/aoc2020/inputs/003.txt")
        .expect("something went wrong");

    let map = parse_map(&input);

    // Part 1
    let start = Instant::now();
    let count = count_trees_encountered(&map);
    let duration = start.elapsed();

    println!("Trees encountered: {}", count);
    println!("Time taken: {:?}", duration);

    // Part 2
    let start = Instant::now();
    let product = product_all_route_methods(&map);
    let duration = start.elapsed();

    println!("Trees encountered all methods: {}", product);
    println!("Time taken: {:?}", duration);
}

fn parse_map(input: &str) -> Vec<Vec<Terrain>> {
    let lines: Vec<&str> = input.lines().collect();

    let mut map = vec![vec![Terrain::Empty; lines[0].chars().count()]; lines.len()];

    lines.iter().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c == '#' {
                map[i][j] = Terrain::Tree;
            } else {
                map[i][j] = Terrain::Empty;
            }
        })
    });

    return map;
}

fn count_trees_encountered(map: &Vec<Vec<Terrain>>) -> usize {
    let mut count = 0;
    let mut col = 0;

    for row in 0..map.len() {
        if map[row][col] == Terrain::Tree {
            count += 1;
        }
        col = (col + 3) % (map[0].len());
    }
    return count;
}

fn product_all_route_methods(map: &Vec<Vec<Terrain>>) -> usize {
    let max_cols = map[0].len();
    let (mut count_a, mut col_a, right_a) = (0, 0, 1);
    let (mut count_b, mut col_b, right_b) = (0, 0, 3);
    let (mut count_c, mut col_c, right_c) = (0, 0, 5);
    let (mut count_d, mut col_d, right_d) = (0, 0, 7);
    let (mut count_e, mut col_e, right_e) = (0, 0, 1);

    for row in 0..map.len() {
        if map[row][col_a] == Terrain::Tree {
            count_a += 1;
        }
        if map[row][col_b] == Terrain::Tree {
            count_b += 1;
        }
        if map[row][col_c] == Terrain::Tree {
            count_c += 1;
        }
        if map[row][col_d] == Terrain::Tree {
            count_d += 1;
        }
        if row % 2 == 0 && map[row][col_e] == Terrain::Tree {
            count_e += 1;
        }
        col_a = (col_a + right_a) % (max_cols);
        col_b = (col_b + right_b) % (max_cols);
        col_c = (col_c + right_c) % (max_cols);
        col_d = (col_d + right_d) % (max_cols);
        if row % 2 == 0 {
            col_e = (col_e + right_e) % (max_cols);
        }
    }
    return count_a * count_b * count_c * count_d * count_e;
}

#[derive(Clone, Debug, PartialEq)]

enum Terrain {
    Empty,
    Tree,
}

#[test]
fn test_question_part_one() {
    let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    let map = parse_map(&input);

    let count = count_trees_encountered(&map);

    assert_eq!(count, 7);
}

#[test]
fn test_question_part_two() {
    let input = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#";

    let map = parse_map(&input);

    let product = product_all_route_methods(&map);

    assert_eq!(product, 336);
}
