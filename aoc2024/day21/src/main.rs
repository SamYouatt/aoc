use std::collections::{HashMap, VecDeque};

use santas_little_helpers::{directions::Direction, grid::Grid};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn paths(start: char, end: char, pad: &Grid<char>) -> Vec<Vec<char>> {
    let start_pos = pad.find_first(&start).unwrap();
    let end_pos = pad.find_first(&end).unwrap();

    let mut paths = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((start_pos, vec![], 0));
    let mut best_cost = usize::MAX;

    while let Some((pos, path, cost)) = queue.pop_front() {
        if cost > best_cost {
            break;
        }

        if pos == end_pos {
            let mut new_path = path.clone();
            new_path.push('A');
            paths.push(new_path);

            best_cost = cost;
            continue;
        }

        for dir in Direction::iterator() {
            let next_pos = pos + dir.delta();
            if !pad.in_bounds(&next_pos) || pad.get(&next_pos) == &' ' {
                continue;
            }

            let key = match dir {
                Direction::Down => 'v',
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Left => '<',
            };

            let mut new_path = path.clone();
            new_path.push(key);

            queue.push_back((next_pos, new_path.clone(), cost + 1));
        }
    }

    paths
}

fn form_code(
    sequence: &[char],
    depth: usize,
    keypad_locations: &mut Vec<char>,
    num_pad: &Grid<char>,
    dir_pad: &Grid<char>,
    on_num_pad: bool,
    cache: &mut HashMap<(Vec<char>, usize, char), usize>,
) -> usize {
    if let Some(in_cache) = cache.get(&(sequence.to_vec(), depth, keypad_locations[depth])) {
        return *in_cache;
    }

    let mut length = 0;

    for key in sequence {
        let paths = match on_num_pad {
            true => paths(keypad_locations[depth], *key, num_pad),
            false => paths(keypad_locations[depth], *key, dir_pad),
        };

        if depth == 0 {
            length += paths[0].len();
        } else {
            length += paths
                .iter()
                .map(|path| {
                    form_code(
                        path,
                        depth - 1,
                        keypad_locations,
                        num_pad,
                        dir_pad,
                        false,
                        cache,
                    )
                })
                .min()
                .unwrap();
        }

        keypad_locations[depth] = *key;
    }

    cache.insert((sequence.to_vec(), depth, keypad_locations[depth]), length);
    length
}

fn part_1(input: &str) -> usize {
    let num_pad = Grid::from_vecs(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ]);

    let dir_pad = Grid::from(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']]);

    let mut total = 0;
    let mut cache = HashMap::new();

    for line in input.lines() {
        let code = line.chars().collect::<Vec<_>>();
        let mut locations = vec!['A', 'A', 'A'];

        let instructions_len = form_code(
            &code,
            2,
            &mut locations,
            &num_pad,
            &dir_pad,
            true,
            &mut cache,
        );
        let numeric_code = line[0..3].parse::<usize>().unwrap();

        total += instructions_len * numeric_code;
    }

    total
}

fn part_2(input: &str) -> usize {
    let num_pad = Grid::from_vecs(vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec![' ', '0', 'A'],
    ]);

    let dir_pad = Grid::from(vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']]);

    let mut total = 0;
    let mut cache = HashMap::new();

    for line in input.lines() {
        let code = line.chars().collect::<Vec<_>>();
        let mut locations = vec!['A'; 26];

        let instructions_len = form_code(
            &code,
            25,
            &mut locations,
            &num_pad,
            &dir_pad,
            true,
            &mut cache,
        );
        let numeric_code = line[0..3].parse::<usize>().unwrap();

        total += instructions_len * numeric_code;
    }

    total
}
