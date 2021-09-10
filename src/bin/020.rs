fn main() {
    let tiles = include_str!("../../inputs/020.txt")
        .split("\n\n")
        .map(|chunk| {
            let (id, tile) = chunk.split_once('\n').unwrap();
            let id: usize = id[5..].trim_end_matches(':').parse().unwrap();
            let pixels: Vec<u8> = tile.bytes().filter(|&b| b != b'\n').collect();

            // get each edge of the tile
            // stored in a u16, it is actually just the first 10 bits that matter
            // as the edge can be turned into a number with each bit representing a pixel
            let top_edge = pixels[0..10].iter().fold(0_u16, |edge, &pixel| {
                edge << 1 | if pixel == b'#' { 1 } else { 0 }
            });
            let left_edge = (0..10).map(|i| pixels[i * 10]).fold(0_u16, |edge, pixel| {
                edge << 1 | if pixel == b'#' { 1 } else { 0 }
            });
            let right_edge = (0..10)
                .map(|i| pixels[10 * (i + 1) - 1])
                .fold(0_u16, |edge, pixel| {
                    edge << 1 | if pixel == b'#' { 1 } else { 0 }
                });
            let bottom_edge = pixels[pixels.len() - 10..pixels.len()]
                .iter()
                .fold(0_u16, |edge, &pixel| {
                    edge << 1 | if pixel == b'#' { 1 } else { 0 }
                });

            (id, [top_edge, left_edge, right_edge, bottom_edge])
        })
        .collect::<Vec<(usize, [u16; 4])>>();

    println!("{:#?}", tiles[0]);
}
