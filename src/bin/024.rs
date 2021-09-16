use std::collections::HashSet;

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

    let flipped = tiles
        .iter()
        .fold(HashSet::new(), |mut flips, coord| {
            if !flips.remove(&coord) {
                flips.insert(coord);
            }
            flips
        })
        .len();

    println!("Part one: {} in {:#?}", flipped, 0);
}
