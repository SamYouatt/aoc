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
    let neighbours: Vec<usize, Vec<usize>> = seat_indexes
        .iter()
        .map(|seat| (0..9)
            .filter(|i|)
        )
        .collect();
}
