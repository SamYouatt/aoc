fn main() {
    let input = include_str!("../../inputs/011.txt");

    let rows = input.lines().count();
    let cols = input.lines().next().map(|line| line.len()).unwrap();

    println!("rows: {}, cols: {}", rows, cols);

    let seat_indexes: Vec<usize> = input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_, pos)| pos == &&b'L')
        .map(|(i, _)| i - (i / cols + 1))
        .collect();

    #[rustfmt::skip]
    let neighbours: Vec<(usize, Vec<usize>)> = seat_indexes
        .iter()
        .map(|seat| (*seat, (0..9)
            .filter(|index| index != &4)
            .map(|index| ((*seat as isize % cols as isize) + index % 3 - 1, (*seat as isize / cols as isize) + index / 3 - 1))
            .filter(|(x, y)| *x >= 0 && *x < cols as isize && *y >= 0 && *y < rows as isize)
            .map(|(x, y)| (y * cols as isize + x) as usize)
            .collect(),
        ))
        .collect();
}

enum State {
    Occupied,
    Empty,
    Floor,
}
