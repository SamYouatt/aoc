use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", both_parts(input).0);
    println!("Part 2: {}", both_parts(input).1);
}

#[derive(Debug)]
struct Trail {
    start: (isize, isize),
    current: (isize, isize),
    previous: HashSet<(isize, isize)>,
}

fn both_parts(input: &str) -> (usize, usize) {
    let mut trails = Vec::new();
    let map: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == '0' {
                        let start = (x as isize, y as isize);
                        trails.push(Trail {
                            start,
                            current: start,
                            previous: HashSet::from([start]),
                        });
                    }

                    char.to_digit(10).unwrap() as isize
                })
                .collect()
        })
        .collect();

    let width = input.lines().nth(1).unwrap().chars().count();
    let height = input.lines().count();

    let mut finished_trails: HashMap<(isize, isize), HashSet<(isize, isize)>> = HashMap::new();
    let mut finished_trail_routes: HashMap<(isize, isize), usize> = HashMap::new();

    while let Some(trail) = trails.pop() {
        let current_grad = map[trail.current.1 as usize][trail.current.0 as usize];

        if current_grad == 9 {
            finished_trails
                .entry(trail.start)
                .or_insert(HashSet::new())
                .insert(trail.current);

            *finished_trail_routes.entry(trail.start).or_insert(0) += 1;

            continue;
        }

        for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = (trail.current.0 + delta.0, trail.current.1 + delta.1);

            if in_bounds(next, width, height)
                && !trail.previous.contains(&next)
                && (map[next.1 as usize][next.0 as usize] - current_grad) == 1
            {
                let mut visited = trail.previous.clone();
                visited.insert(next);

                trails.push(Trail {
                    start: trail.start,
                    current: next,
                    previous: visited,
                });
            }
        }
    }

    (
        finished_trails.iter().fold(0, |acc, (_k, v)| acc + v.len()),
        finished_trail_routes.iter().fold(0, |acc, (_k, v)| acc + v),
    )
}

fn in_bounds(point: (isize, isize), width: usize, height: usize) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < width as isize && point.1 < height as isize
}
