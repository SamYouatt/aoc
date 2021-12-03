#[derive(Debug)]
pub struct Command {
    direction: Direction,
    amount: usize,
}

#[derive(Debug)]
enum Direction {
    Forward,
    Down,
    Up,
}

struct Position {
    horizontal: usize,
    depth: usize,
}

#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|cmd| {
            return match cmd {
                ("forward", _) => Command {
                    direction: Direction::Forward,
                    amount: cmd.1.parse().unwrap(),
                },
                ("down", _) => Command {
                    direction: Direction::Down,
                    amount: cmd.1.parse().unwrap(),
                },
                ("up", _) => Command {
                    direction: Direction::Up,
                    amount: cmd.1.parse().unwrap(),
                },
                (_, _) => panic!("Bad line"),
            };
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Command]) -> usize {
    let position = input
        .iter()
        .fold((0_usize, 0_usize), |(f, d), cmd| match cmd.direction {
            Direction::Forward => (f + cmd.amount, d),
            Direction::Down => (f, d + cmd.amount),
            Direction::Up => (f, d - cmd.amount),
        });
    position.0 * position.1
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Command]) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn test_part1() {
        let commands = parse_input(&INPUT);
        assert_eq!(solve_part1(&commands), 150);
    }
}
