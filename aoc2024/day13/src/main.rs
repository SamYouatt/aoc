fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> isize {
    input
        .split("\n\n")
        .map(|chunk| {
            let mut lines = chunk.lines();
            let (ax, ay) = parse_button(lines.next().unwrap());
            let (bx, by) = parse_button(lines.next().unwrap());
            let (goal_x, goal_y) = parse_goal(lines.next().unwrap());

            (ax, ay, bx, by, goal_x, goal_y)
        })
        .map(|(ax, ay, bx, by, goal_x, goal_y)| solve(ax, ay, bx, by, goal_x, goal_y))
        .sum()
}

fn part_2(input: &str) -> isize {
    input
        .split("\n\n")
        .map(|chunk| {
            let mut lines = chunk.lines();
            let (ax, ay) = parse_button(lines.next().unwrap());
            let (bx, by) = parse_button(lines.next().unwrap());
            let (goal_x, goal_y) = parse_goal(lines.next().unwrap());

            (ax, ay, bx, by, goal_x + 10000000000000, goal_y + 10000000000000)
        })
        .map(|(ax, ay, bx, by, goal_x, goal_y)| solve(ax, ay, bx, by, goal_x, goal_y))
        .sum()
}

// Solve these simul equations
//a * a_x + b * b_x = goal_x
//a * a_y + b * b_y = goal_y
// Rearranging and subbing gives:
// b = (goal_y * a_x - goal_x * a_y) / (b_y * a_x - b_x * a_y)
// a = (goal_x - b * a_y) / a_x
fn solve(a_x: isize, a_y: isize, b_x: isize, b_y: isize, goal_x: isize, goal_y: isize) -> isize {
    let b = (goal_y * a_x - goal_x * a_y) / (b_y * a_x - b_x * a_y);
    let a = (goal_x - b * b_x) / a_x;

    // if this doesn't match then the solution isn't possible
    if (a * a_x + b * b_x, a * a_y + b * b_y) != (goal_x, goal_y) {
        return 0;
    }

    return 3 * a + b;
}

fn parse_button(raw: &str) -> (isize, isize) {
    let (left, right) = raw.split_once(", ").unwrap();

    let (_, x) = left.split_once('+').unwrap();
    let x = x.parse::<isize>().unwrap();

    let (_, y) = right.split_once('+').unwrap();
    let y = y.parse::<isize>().unwrap();

    (x, y)
}

fn parse_goal(raw: &str) -> (isize, isize) {
    let (left, right) = raw.split_once(", ").unwrap();

    let (_, x) = left.split_once('=').unwrap();
    let x = x.parse::<isize>().unwrap();

    let (_, y) = right.split_once('=').unwrap();
    let y = y.parse::<isize>().unwrap();

    (x, y)
}
