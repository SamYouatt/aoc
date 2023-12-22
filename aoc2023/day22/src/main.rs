use std::collections::HashSet;

use aoc_util::coordinate::Coordinate3;

fn main() {
    let input = include_str!("input.txt");

    let (answer1, answer2) = both_parts(input);
    println!("Part 1: {answer1}");
    println!("Part 2: {answer2}");
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Brick {
    points: HashSet<Coordinate3>,
    footprint: HashSet<Coordinate3>,
}

fn both_parts(input: &str) -> (usize, usize) {
    let mut bricks: Vec<_> = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").unwrap();
            let start = parse_coord(start);
            let end = parse_coord(end);

            build_brick(&start, &end)
        })
        .collect();

    // sort the bricks by the z index of the lowest point
    bricks.sort_by(|brick_a, brick_b| {
        let lowest_a = brick_a
            .footprint
            .iter()
            .min_by(|foot_a, foot_b| foot_a.z.cmp(&foot_b.z))
            .unwrap()
            .z;
        let lowest_b = brick_b
            .footprint
            .iter()
            .min_by(|foot_a, foot_b| foot_a.z.cmp(&foot_b.z))
            .unwrap()
            .z;

        lowest_a.cmp(&lowest_b)
    });

    let (settled_bricks, _) = simulate(&bricks);

    let mut can_be_removed = 0;
    let mut total_fell = 0;

    for brick_to_remove in &settled_bricks {
        let without_brick: Vec<_> = settled_bricks
            .iter()
            .filter(|brick| brick != &brick_to_remove)
            .map(|brick| brick.to_owned())
            .collect();

        let (_, num_moved) = simulate(&without_brick);

        if num_moved == 0 {
            can_be_removed += 1;
        }

        total_fell += num_moved;
    }

    (can_be_removed, total_fell)
}

fn parse_coord(input: &str) -> Coordinate3 {
    let (x, rest) = input.split_once(",").unwrap();
    let (y, z) = rest.split_once(",").unwrap();

    Coordinate3 {
        x: x.parse::<isize>().unwrap(),
        y: y.parse::<isize>().unwrap(),
        z: z.parse::<isize>().unwrap(),
    }
}

fn build_brick(start: &Coordinate3, end: &Coordinate3) -> Brick {
    let mut points = HashSet::new();
    let mut footprint = HashSet::new();

    if end.x > start.x {
        for x in start.x..=end.x {
            points.insert(Coordinate3::new(x, start.y, start.z));
            footprint.insert(Coordinate3::new(x, start.y, start.z));
        }
    } else if end.y > start.y {
        for y in start.y..=end.y {
            points.insert(Coordinate3::new(start.x, y, start.z));
            footprint.insert(Coordinate3::new(start.x, y, start.z));
        }
    } else if end.z > start.z {
        // only upwards pointing blocks have different bases
        for z in start.z..=end.z {
            points.insert(Coordinate3::new(start.x, start.y, z));
        }
        footprint.insert(Coordinate3::new(start.x, start.y, start.z));
    } else {
        points.insert(Coordinate3::new(start.x, start.y, start.z));
        footprint.insert(Coordinate3::new(start.x, start.y, start.z));
    }

    Brick { points, footprint }
}

fn simulate(bricks: &Vec<Brick>) -> (Vec<Brick>, usize) {
    let mut bricks = bricks.to_owned();
    let mut world = HashSet::new();

    for brick in &bricks {
        for &point in &brick.points {
            world.insert(point);
        }
    }

    let mut brick_fell = true;
    // needs to be hashset cos a brick might move twice as part of the simulation
    // but we only want to count it once, don't care how much it moved
    let mut moved_bricks = HashSet::new();

    while brick_fell {
        brick_fell = false;

        for i in 0..bricks.len() {
            let brick = &mut bricks[i];
            let moved_down = move_down(&brick);

            if moved_down
                .footprint
                .iter()
                .any(|point| world.contains(point) || point.z == 0)
            {
                continue;
            }

            for point in brick.points.iter() {
                world.remove(point);
            }

            for &point in &moved_down.points {
                world.insert(point);
            }

            brick_fell = true;
            moved_bricks.insert(i);
            *brick = moved_down;
        }
    }

    (bricks, moved_bricks.len())
}

fn move_down(brick: &Brick) -> Brick {
    let points: HashSet<_> = brick
        .points
        .iter()
        .map(|point| Coordinate3::new(point.x, point.y, point.z - 1))
        .collect();

    let footprint: HashSet<_> = brick
        .footprint
        .iter()
        .map(|point| Coordinate3::new(point.x, point.y, point.z - 1))
        .collect();

    Brick { points, footprint }
}
