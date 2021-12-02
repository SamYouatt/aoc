use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn main() {
    let tiles: Vec<(isize, isize)> = include_bytes!("../../inputs/024.txt")
        .split_inclusive(|&b| b == b'\n')
        .map(|line| {
            line.windows(2)
                .fold((0, 0), |(x, y), instruction| match instruction {
                    [b'e', _] => (x, y + 1),
                    [b'w', _] => (x, y - 1),
                    [b's', b'e'] => (x + 1, y),
                    [b's', b'w'] => (x + 1, y + 1),
                    [b'n', b'e'] => (x - 1, y - 1),
                    [b'n', b'w'] => (x - 1, y),
                    _ => panic!(),
                })
        })
        .collect();

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(&tiles), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} in {:#?}", part_two(&tiles), start.elapsed());
}

fn part_one(tiles: &[(isize, isize)]) -> usize {
    tiles
        .iter()
        .fold(HashSet::new(), |mut flips, coord| {
            if !flips.remove(&coord) {
                flips.insert(coord);
            }
            flips
        })
        .len()
}

fn part_two(tiles: &[(isize, isize)]) -> usize {
    let mut previous_blacks = tiles
        .iter()
        .copied()
        .fold(HashSet::new(), |mut flips, coord| {
            if !flips.remove(&coord) {
                flips.insert(coord);
            }
            flips
        });

    // the relative coordinates that neighbours can be in the skewed hexagonal space
    let neighbour_offsets = [(0, 1), (0, -1), (1, 1), (1, 0), (-1, 0), (-1, -1)];

    // instead of keeping track of all tiles we just need to track the current black tiles
    let mut current_blacks: HashSet<(isize, isize)> = HashSet::new();

    // we then keep a reference of a given tile's coordinate and its neighbour count
    let mut neighbour_counts: HashMap<(isize, isize), usize> = HashMap::new();

    (0..100).for_each(|_| {
        // go through each previously black tile and go through its neighbours and see
        // how many neighbours it has
        previous_blacks.iter().for_each(|(x, y)| {
            neighbour_offsets.iter().for_each(|(dx, dy)| {
                *neighbour_counts.entry((x + dx, y + dy)).or_default() += 1;
            })
        });

        current_blacks = HashSet::new();

        // drain the neighbour counts and then based on the number of neighbours and
        // the tile colour, decide whether to change its state
        // if it will now be black, add it to the list of blacks
        neighbour_counts.drain().for_each(|(coord, count)| {
            if previous_blacks.contains(&coord) && count <= 2
                || !previous_blacks.contains(&coord) && count == 2
            {
                current_blacks.insert(coord);
            }
        });

        // update previous to current
        std::mem::swap(&mut previous_blacks, &mut current_blacks);
    });

    previous_blacks.len()
}
