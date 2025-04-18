use std::ops::{Add, Sub};

use aoc_util::coordinate::Coordinate;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input, 200000000000000.0, 400000000000000.0);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

#[derive(Debug)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn apply(&self, pos: &Coordinate) -> Coordinate {
        Coordinate::new(pos.x + self.x, pos.y + self.y)
    }
}

#[derive(Debug)]
struct Intersect {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Hail {
    p1: Coordinate,
    p2: Coordinate,
    v: Vector,
}

#[derive(Debug, Clone, Copy)]
struct Coordinate3 {
    x: isize,
    y: isize,
    z: isize,
}

impl Sub for Coordinate3 {
    type Output = Coordinate3;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Add for Coordinate3 {
    type Output = Coordinate3;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Coordinate3 {
    fn apply(&self, other: &Coordinate3) -> Coordinate3 {
        Coordinate3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn multiply(&self, times: isize) -> Coordinate3 {
        Coordinate3 {
            x: self.x * times,
            y: self.y * times,
            z: self.z * times,
        }
    }
}

#[derive(Debug)]
struct Hail3 {
    pos: Coordinate3,
    vel: Coordinate3,
}

impl Hail {
    fn is_future(&self, pos: &Intersect) -> bool {
        let orig_x = self.p1.x as f64;
        let orig_y = self.p1.y as f64;

        if self.v.x > 0 && orig_x > pos.x {
            return false;
        }
        if self.v.y > 0 && orig_y > pos.y {
            return false;
        }
        if self.v.x < 0 && orig_x < pos.x {
            return false;
        }
        if self.v.y < 0 && orig_y < pos.y {
            return false;
        }

        true
    }
}

fn part_1(input: &str, min: f64, max: f64) -> usize {
    let hail: Vec<_> = input
        .lines()
        .map(|line| {
            let (pos, vec) = line.split_once(" @ ").unwrap();
            let position_points: Vec<_> = pos
                .replace(" ", "")
                .split(",")
                .map(|num| num.parse::<isize>().unwrap())
                .collect();
            let vector_points: Vec<_> = vec
                .replace(" ", "")
                .split(",")
                .map(|num| num.parse::<isize>().unwrap())
                .collect();

            let p1 = Coordinate::new(position_points[0], position_points[1]);
            let v = Vector {
                x: vector_points[0],
                y: vector_points[1],
            };
            let p2 = v.apply(&p1);

            Hail { p1, p2, v }
        })
        .collect();

    let mut intersections = 0;

    for i in 0..hail.len() {
        for j in (i + 1)..hail.len() {
            let hail_a = &hail[i];
            let hail_b = &hail[j];

            match intersect(&hail_a.p1, &hail_a.p2, &hail_b.p1, &hail_b.p2) {
                Some(inter) => {
                    if !(inter.x >= min && inter.x <= max && inter.y >= min && inter.y <= max) {
                        continue;
                    }

                    if !hail_a.is_future(&inter) || !hail_b.is_future(&inter) {
                        continue;
                    }

                    intersections += 1;
                }
                None => {}
            }
        }
    }

    intersections
}

fn part_2(input: &str) -> usize {
    let hail: Vec<_> = input
        .lines()
        .map(|line| {
            let (pos, vec) = line.split_once(" @ ").unwrap();
            let position_points: Vec<_> = pos
                .replace(" ", "")
                .split(",")
                .map(|num| num.parse::<isize>().unwrap())
                .collect();
            let vector_points: Vec<_> = vec
                .replace(" ", "")
                .split(",")
                .map(|num| num.parse::<isize>().unwrap())
                .collect();

            let pos = Coordinate3 {
                x: position_points[0],
                y: position_points[1],
                z: position_points[2],
            };
            let vel = Coordinate3 {
                x: vector_points[0],
                y: vector_points[1],
                z: vector_points[2],
            };

            Hail3 { pos, vel }
        })
        .collect();

    let x_velocity = find_only_velocity(&hail, Axis::X);
    let y_velocity = find_only_velocity(&hail, Axis::Y);
    let z_velocity = find_only_velocity(&hail, Axis::Z);

    let rock_velocity = Coordinate3 {
        x: x_velocity,
        y: y_velocity,
        z: z_velocity,
    };

    // adjust the velocities of two other lines by the vector of the rock. Now the persepective is
    // that the rock is stationary and by working out the point when the adjusted lines intersect,
    // can then derive the position the rock would be in
    let first_adjusted_hail = Hail3 {
        pos: hail[0].pos,
        vel: hail[0].vel - rock_velocity,
    };
    let first_adjusted_hail_p2 = first_adjusted_hail.pos.apply(&first_adjusted_hail.vel);

    let second_adjusted_hail = Hail3 {
        pos: hail[1].pos,
        vel: hail[1].vel - rock_velocity,
    };
    let second_adjusted_hail_p2 = second_adjusted_hail.pos.apply(&second_adjusted_hail.vel);

    // grab the raw ua out of the intersect, this is essentially the 'time' along the line that the
    // intersection is found
    let intersect_time = raw_intersect_point(
        &first_adjusted_hail.pos,
        &first_adjusted_hail_p2,
        &second_adjusted_hail.pos,
        &second_adjusted_hail_p2,
    )
    .unwrap();

    let rock_start =
        first_adjusted_hail.pos + (first_adjusted_hail.vel.multiply(intersect_time as isize));

    (rock_start.x + rock_start.y + rock_start.z) as usize
}

enum Axis {
    X,
    Y,
    Z,
}

// consider only one single axis and find the only? possible vector along that axis
fn find_only_velocity(hail: &Vec<Hail3>, axis: Axis) -> isize {
    let mut results = HashSet::new();

    let hail_axis = hail
        .iter()
        .map(|h| match axis {
            Axis::X => (h.pos.x, h.vel.x),
            Axis::Y => (h.pos.y, h.vel.y),
            Axis::Z => (h.pos.z, h.vel.z),
        })
        .sorted_by(|a, b| a.1.cmp(&b.1))
        .group_by(|&hail| hail.1);

    for (hail_axis, matching_velocities) in &hail_axis {
        let matching_velocities = matching_velocities.collect_vec();
        if matching_velocities.len() == 1 {
            // need to have multiple lines of the same velocity in order to calculate the speed
            // needed to hit both of them
            continue;
        }

        // need to find the distance between those two lines so that can work out all the
        // velocities that can reach between those lines in whole integer increments
        let first_point = matching_velocities[0];
        let second_point = matching_velocities[1];
        let distance_between = (second_point.0 - first_point.0).abs();

        // the factors get all the possible integer velocities that would work at some integer time
        // value
        let velocity_factors = get_factors(distance_between);

        // annoyingly the velocities could be either negative or positive with this way
        let possible_velocities = velocity_factors
            .iter()
            .flat_map(|&factor| [factor, -factor])
            .map(|factor| hail_axis + factor)
            .collect_vec();

        match results.len() {
            0 => results.extend(possible_velocities),
            _ => {
                let possible_velocities = HashSet::from_iter(possible_velocities);
                results = results
                    .intersection(&possible_velocities)
                    .cloned()
                    .collect();
            }
        }

        if results.len() == 1 {
            return *results.iter().next().unwrap();
        }
    }

    panic!("Didn't find a value for this axis")
}

fn get_factors(num: isize) -> Vec<isize> {
    let root = f64::sqrt(num as f64) as isize + 1;

    (1..=root)
        .into_par_iter()
        .filter(|x| num % x == 0)
        .collect()
}

fn intersect(
    a1: &Coordinate,
    a2: &Coordinate,
    b1: &Coordinate,
    b2: &Coordinate,
) -> Option<Intersect> {
    let (x1, x2, y1, y2) = (a1.x as f64, a2.x as f64, a1.y as f64, a2.y as f64);
    let (x3, x4, y3, y4) = (b1.x as f64, b2.x as f64, b1.y as f64, b2.y as f64);

    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

    // parallel
    if denominator == 0.0 {
        return None;
    }

    let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;

    let x = x1 + ua * (x2 - x1);
    let y = y1 + ua * (y2 - y1);

    Some(Intersect { x, y })
}

fn raw_intersect_point(
    a1: &Coordinate3,
    a2: &Coordinate3,
    b1: &Coordinate3,
    b2: &Coordinate3,
) -> Option<f64> {
    let (x1, x2, y1, y2) = (a1.x as f64, a2.x as f64, a1.y as f64, a2.y as f64);
    let (x3, x4, y3, y4) = (b1.x as f64, b2.x as f64, b1.y as f64, b2.y as f64);

    let denominator = (y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1);

    // parallel
    if denominator == 0.0 {
        return None;
    }

    // there is technically a second solution at the minus denominator point as well but hopefully
    // I won't need it
    let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3)) / denominator;

    Some(ua)
}

#[test]
fn part_1_example() {
    let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

    let answer = part_1(input, 7.0, 27.0);

    assert_eq!(answer, 2);
}
