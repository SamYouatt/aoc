#[derive(Debug)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }

    fn move_in(&self, direction: &Direction) -> Coord {
        match direction {
            Direction::North => Coord::new(self.x, self.y + 1),
            Direction::East => Coord::new(self.x + 1, self.y),
            Direction::South => Coord::new(self.x, self.y - 1),
            Direction::West => Coord::new(self.x - 1, self.y),
        }
    }
}

impl Direction {
    fn parse(direction: &str) -> Direction {
        match direction {
            "R" => Direction::East,
            "U" => Direction::North,
            "L" => Direction::West,
            "D" => Direction::South,
            _ => panic!("Unknown direction"),
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
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (direction, rest) = line.split_once(" ").unwrap();
            let (steps, _rest) = rest.split_once(" (").unwrap();

            Instruction {
                direction: Direction::parse(direction),
                steps: steps.parse::<usize>().unwrap(),
            }
        })
        .collect();

    let trench = dig_trench(&instructions);

    let area = shoelace_area(&trench);

    area
}

fn part_2(input: &str) -> isize {
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let (_rest, hex) = line.split_once("(").unwrap();
            let hex = hex.replace(")", "");

            let hex_digits: String = hex.chars().collect();

            let distance = usize::from_str_radix(&hex_digits[1..6], 16).unwrap();
            let direction = match &hex_digits.chars().last().unwrap() {
                '0' => Direction::East,
                '1' => Direction::South,
                '2' => Direction::West,
                '3' => Direction::North,
                _ => panic!("Unknown direction"),
            };

            Instruction {
                steps: distance,
                direction,
            }
        })
        .collect();

    let trench = dig_trench(&instructions);

    let area = shoelace_area(&trench);

    area
}

fn dig_trench(instructions: &Vec<Instruction>) -> Vec<Coord> {
    let mut position = Coord::new(0, 0);
    let mut trench = Vec::from_iter([position]);

    for instruction in instructions {
        for _step in 0..instruction.steps {
            position = position.move_in(&instruction.direction);

            if position.x == 0 && position.y == 0 {
                continue;
            }

            trench.push(position);
        }
    }

    trench
}

fn shoelace_area(points: &Vec<Coord>) -> isize {
    let mut shoelace_sum: isize = 0;

    for (i, point) in points.iter().enumerate() {
        let next = if i < points.len() - 1 { i + 1 } else { 0 };

        shoelace_sum += (point.x * points[next].y) - (point.y * points[next].x);
    }

    let lattice_area = isize::abs(shoelace_sum) / 2;
    let perimeter = points.len() as isize;

    lattice_area + (perimeter / 2) + 1
}

#[test]
fn test_shoelace_box() {
    let points = vec![
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(3, 0),
        Coord::new(3, 1),
        Coord::new(3, 2),
        Coord::new(3, 3),
        Coord::new(2, 3),
        Coord::new(1, 3),
        Coord::new(0, 3),
        Coord::new(0, 2),
        Coord::new(0, 1),
    ];

    // # # # #
    // #     #
    // #     #
    // # # # #
    let area = shoelace_area(&points);

    assert_eq!(area, 16);
}
