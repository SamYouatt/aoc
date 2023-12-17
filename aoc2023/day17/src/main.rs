use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

impl Direction {
    fn x_off(&self) -> isize {
        match self {
            Direction::North => 0,
            Direction::South => 0,
            Direction::East => 1,
            Direction::West => -1,
            _ => panic!(),
        }
    }

    fn y_off(&self) -> isize {
        match self {
            Direction::North => -1,
            Direction::South => 1,
            Direction::East => 0,
            Direction::West => 0,
            _ => panic!(),
        }
    }

    fn is_opposite(&self, other: &Direction) -> bool {
        match (self, other) {
            (Direction::North, Direction::South) | (Direction::South, Direction::North) => true,
            (Direction::East, Direction::West) | (Direction::West, Direction::East) => true,
            _ => false,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

fn part_1(input: &str) -> isize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|ch| (*ch - b'0') as usize)
                .collect()
        })
        .collect();

    find_path(&grid, 1, 3)
}

fn part_2(input: &str) -> isize {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|ch| (*ch - b'0') as usize)
                .collect()
        })
        .collect();

    find_path(&grid, 4, 10)
}

fn find_path(grid: &Vec<Vec<usize>>, min_step: isize, max_step: isize) -> isize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut best_dist: HashMap<(Coord, Direction), isize> = HashMap::new();
    let mut unvisited: BinaryHeap<(_, (Coord, Direction))> = BinaryHeap::new();

    // binary heap is a max heap but can use the std::cmp::Reverse to turn it into a min heap!
    unvisited.push((Reverse(0), (Coord::new(0, 0), Direction::None)));

    while let Some((rev_cost, (position, direction))) = unvisited.pop() {
        // reached destination
        if position.x == (cols - 1) as isize && position.y == (rows - 1) as isize {
            return rev_cost.0;
        }

        for next_direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if next_direction == direction || next_direction.is_opposite(&direction) {
                continue;
            }

            let mut next_cost = rev_cost.0;

            for steps in 1..=max_step {
                let next_y = position.y + next_direction.y_off() * steps;
                let next_x = position.x + next_direction.x_off() * steps;

                if next_y >= rows as isize || next_y < 0 || next_x >= cols as isize || next_x < 0 {
                    continue;
                }

                next_cost += grid[next_y as usize][next_x as usize] as isize;

                // need to count the costs as we pass over them but we have't finished our movement
                // yet
                if steps < min_step {
                    continue;
                }

                let key = (Coord::new(next_x, next_y), next_direction);

                if !best_dist.contains_key(&key) || best_dist[&key] > next_cost {
                    best_dist.insert(key, next_cost);
                    unvisited.push((Reverse(next_cost), key));
                }
            }
        }
    }

    panic!("Somehow no next steps but did not reach destination");
}

#[test]
fn part_1_test() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|ch| (*ch - b'0') as usize)
                .collect()
        })
        .collect();

    let best_cost = find_path(&grid, 1, 3);

    assert_eq!(best_cost, 102);
}

#[test]
fn part_2_test() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|ch| (*ch - b'0') as usize)
                .collect()
        })
        .collect();

    let best_cost = find_path(&grid, 4, 10);

    assert_eq!(best_cost, 94);
}

#[test]
fn part_2_test_2() {
    let input = "111111111111
999999999991
999999999991
999999999991
999999999991";

    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|ch| (*ch - b'0') as usize)
                .collect()
        })
        .collect();

    let best_cost = find_path(&grid, 4, 10);

    assert_eq!(best_cost, 71);
}
