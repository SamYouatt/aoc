use aoc_util::coordinate::Coordinate;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input, 200000000000000.0, 400000000000000.0);
    println!("Part 1: {answer1}");
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
