use std::collections::HashSet;

use santas_little_helpers::{
    coord,
    coord::{Coord, Delta},
    delta,
};

fn main() {
    let input = include_str!("input.txt");
    let robots = parse_robots(input);

    println!("Part 1: {}", part_1(robots.clone()));
    println!("Part 2: {}", part_2(robots));
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Coord,
    vel: Delta,
}

fn part_1(mut robots: Vec<Robot>) -> usize {
    let width = 101;
    let height = 103;

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            let new_pos = apply_wrapping(robot.pos, robot.vel, width, height);
            robot.pos = new_pos;
        }
    }

    let mid_x = (width / 2) as isize;
    let mid_y = (height / 2) as isize;
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

    for robot in robots {
        if robot.pos.x < mid_x && robot.pos.y < mid_y {
            q1 += 1;
        } else if robot.pos.x > mid_x && robot.pos.y < mid_y {
            q2 += 1;
        } else if robot.pos.x < mid_x && robot.pos.y > mid_y {
            q3 += 1;
        } else if robot.pos.x > mid_x && robot.pos.y > mid_y {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4
}

fn part_2(mut robots: Vec<Robot>) -> usize {
    let width = 101;
    let height = 103;

    let mut iteration = 0;
    let mut all_robots = Vec::new();
    let mut hash_robots = HashSet::new();

    while all_robots.len() != hash_robots.len() || iteration == 0 {
        for robot in robots.iter_mut() {
            let new_pos = apply_wrapping(robot.pos, robot.vel, width, height);
            robot.pos = new_pos;
        }

        all_robots = robots.iter().map(|rob| rob.pos).collect();
        hash_robots = HashSet::from_iter(all_robots.clone());
        iteration += 1;
    }

    iteration
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();

            let (_, robot) = left.split_once('=').unwrap();
            let (x, y) = robot.split_once(',').unwrap();
            let (x, y) = (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap());

            let (_, vel) = right.split_once('=').unwrap();
            let (vx, vy) = vel.split_once(',').unwrap();
            let (vx, vy) = (vx.parse::<isize>().unwrap(), vy.parse::<isize>().unwrap());

            Robot {
                pos: coord!(x, y),
                vel: delta!(vx, vy),
            }
        })
        .collect()
}

fn apply_wrapping(pos: Coord, delta: Delta, width: usize, height: usize) -> Coord {
    let new = pos.apply_delta(delta);
    let wrapped_x = new.x.rem_euclid(width as isize);
    let wrapped_y = new.y.rem_euclid(height as isize);

    coord!(wrapped_x, wrapped_y)
}
