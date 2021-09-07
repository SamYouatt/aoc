use std::time::Instant;

use itertools::Itertools;

fn main() {
    let input: Vec<Vec<bool>> = include_bytes!("../../inputs/017.txt")
        .split(|&byte| byte == b'\n')
        .map(|line| { line.iter().map(|c| matches!(c, b'#')) }.collect())
        .collect();

    let start = Instant::now();
    println!(
        "Part one: {} in {:#?}",
        part_one(&input, 6),
        start.elapsed()
    );

    let start = Instant::now();
    println!(
        "Part two: {} in {:#?}",
        part_two(&input, 6),
        start.elapsed()
    );
}

fn part_one(input: &[Vec<bool>], cycles: usize) -> usize {
    // the initial length of the size of the square slice, in this case 8
    let start_side_length = input.len();
    // a new row added to each side per cycle so the maximum slice side length is number of cycles * 2 + original
    let max_side_length = cycles * 2 + start_side_length;

    // To turn integers 0-26 into 3D coordinates we need some equations
    // bearing in mind that divisions are quotient divisions because num is integer
    // these are the equations
    // with this system, the middle value is number 13
    // x => num % 3 - 1
    // y => num / 3 % 3 - 1
    // z => num / 9 - 1
    let neighbour_offsets: Vec<(isize, isize, isize)> = (0..27)
        .filter(|&num| num != 13)
        .map(|num| (num % 3 - 1, num / 3 % 3 - 1, num / 9 - 1))
        .collect();

    // the initial 8x8 slice is positioned in the middle of a big array so we need to know the origin
    let origin = max_side_length / 2;

    let mut current_state =
        vec![vec![vec![false; max_side_length + 1]; max_side_length + 1]; max_side_length / 2 + 1];
    let mut previous_state = current_state.clone();

    (0..start_side_length)
        .cartesian_product(0..start_side_length)
        .for_each(|(x, y)| {
            current_state[0][origin - start_side_length / 2 + y]
                [origin - start_side_length / 2 + x] = input[y][x]
        });

    for cycle in 0..cycles {
        std::mem::swap(&mut current_state, &mut previous_state);

        let current_side_length = start_side_length + cycle * 2;
        // iterate over every cell and run the updating state logic
        for z in 0..=cycle + 1 {
            for y in 0..=current_side_length {
                for x in 0..=current_side_length {
                    let (y, x) = (
                        (origin - current_side_length / 2 + y),
                        (origin - current_side_length / 2 + x),
                    );
                    // calculate the number of active neighbours in the vicinity
                    // works by applying the neighbour offsets to the value
                    // the z can be abs because I am only considering it in one direction
                    let num_active = neighbour_offsets
                        .iter()
                        .map(|&offset| {
                            (
                                (x as isize + offset.0) as usize,
                                (y as isize + offset.1) as usize,
                                (z as isize + offset.2).abs() as usize,
                            )
                        })
                        .filter(|(x, y, z)| previous_state[*z][*y][*x])
                        .count();

                    // this controls whether the cell will update its state
                    current_state[z][y][x] = if previous_state[z][y][x] {
                        (2..=3).contains(&num_active)
                    } else {
                        num_active == 3
                    };
                }
            }
        }
    }

    2 * current_state
        .iter()
        .flat_map(|y| y.iter().flat_map(|x| x.iter().filter(|&state| *state)))
        .count()
        - current_state[0]
            .iter()
            .flat_map(|x| x.iter().filter(|&state| *state))
            .count()
}

fn part_two(input: &[Vec<bool>], cycles: usize) -> usize {
    let start_side_length = input.len();
    let max_side_length = cycles * 2 + start_side_length;

    // same as before but with additional formula for the 4th dimension
    let neighbor_offsets: Vec<(isize, isize, isize, isize)> = (0..3 * 3 * 3 * 3)
        .filter(|&i| i != 3 * 3 * 3 * 3 / 2)
        .map(|i| (i % 3 - 1, i / 3 % 3 - 1, i / 9 % 3 - 1, i / 27 - 1))
        .collect();

    let origin = max_side_length / 2;

    let mut current_state = vec![
        vec![
            vec![vec![false; max_side_length + 1]; max_side_length + 1];
            max_side_length / 2 + 1
        ];
        max_side_length / 2 + 1
    ];
    let mut previous_state = current_state.clone();

    (0..start_side_length)
        .cartesian_product(0..start_side_length)
        .for_each(|(x, y)| {
            current_state[0][0][origin - start_side_length / 2 + y]
                [origin - start_side_length / 2 + x] = input[y][x]
        });

    for cycle in 0..cycles {
        std::mem::swap(&mut current_state, &mut previous_state);

        // same as before but including the extra dimension
        let size = start_side_length + cycle * 2;
        for w in 0..=cycle + 1 {
            for z in 0..=cycle + 1 {
                for y in 0..=size {
                    for x in 0..size {
                        let (y, x) = (origin - size / 2 + y, origin - size / 2 + x);
                        let o = neighbor_offsets
                            .iter()
                            .map(|&r| {
                                (
                                    (x as isize + r.0) as usize,
                                    (y as isize + r.1) as usize,
                                    (z as isize + r.2).abs() as usize,
                                    (w as isize + r.3).abs() as usize,
                                )
                            })
                            .filter(|(x, y, z, w)| previous_state[*w][*z][*y][*x])
                            .count();
                        current_state[w][z][y][x] = if previous_state[w][z][y][x] {
                            o == 2 || o == 3
                        } else {
                            o == 3
                        };
                    }
                }
            }
        }
    }

    4 * (current_state
        .iter()
        .map(|y| {
            y.iter()
                .flat_map(|z| z.iter().flat_map(|w| w.iter().filter(|&c| *c)))
                .count()
        })
        .sum::<usize>()
        - current_state
            .iter()
            .map(|x| x[0].iter().flat_map(|z| z.iter().filter(|&c| *c)).count())
            .sum::<usize>())
}

#[test]
fn part_one_test() {
    let input: Vec<Vec<bool>> = ".#.\n..#\n###"
        .as_bytes()
        .split(|&byte| byte == b'\n')
        .map(|line| { line.iter().map(|c| matches!(c, b'#')) }.collect())
        .collect();

    assert_eq!(part_one(&input, 6), 112);
}
