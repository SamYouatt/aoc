use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

#[derive(Debug)]
struct Trail {
    start: (isize, isize),
    current: (isize, isize),
    length: usize,
    previous: HashSet<(isize, isize)>,
}

fn part_1(input: &str) -> usize {
    let mut starts = Vec::new();
    let map: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    if char == '0' {
                        starts.push((x as isize, y as isize));
                    }
                    char.to_digit(10).unwrap() as isize
                })
                .collect()
        })
        .collect();

    let width = input.lines().nth(1).unwrap().chars().count();
    let height = input.lines().count();

    let mut trails = starts
        .into_iter()
        .map(|start| Trail {
            start,
            current: start,
            length: 0,
            previous: HashSet::from([start]),
        })
        .collect::<Vec<_>>();

    let mut finished_trails: HashMap<(isize, isize), HashSet<(isize, isize)>> = HashMap::new();

    while let Some(trail) = trails.pop() {
        if map[trail.current.1 as usize][trail.current.0 as usize] == 9 {
            finished_trails
                .entry(trail.start)
                .or_insert(HashSet::new())
                .insert(trail.current);
            continue;
        }

        for delta_y in -1_isize..2 {
            for delta_x in -1_isize..2 {
                if (delta_y == 0 && delta_x == 0) || (delta_y != 0 && delta_x != 0) {
                    continue;
                }

                let next = (trail.current.0 + delta_x, trail.current.1 + delta_y);
                if in_bounds(next, width, height)
                    && !trail.previous.contains(&next)
                    && (map[next.1 as usize][next.0 as usize]
                        - map[trail.current.1 as usize][trail.current.0 as usize])
                        == 1
                {
                    let mut visited = trail.previous.clone();
                    visited.insert(next);

                    trails.push(Trail {
                        start: trail.start,
                        current: next,
                        length: trail.length + 1,
                        previous: visited,
                    });
                }
            }
        }
    }

    finished_trails.iter().fold(0, |acc, (_k, v)| acc + v.len())
}

fn in_bounds(point: (isize, isize), width: usize, height: usize) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < width as isize && point.1 < height as isize
}
