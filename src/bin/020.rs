use std::{collections::HashMap, time::Instant};

struct Tile {
    id: usize,
    edges: [u16; 4],
}

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

            Tile {
                id,
                edges: [top_edge, left_edge, right_edge, bottom_edge],
            }
        })
        .collect::<Vec<Tile>>();

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(&tiles), start.elapsed());
}

fn part_one(tiles: &[Tile]) -> usize {
    let edge_matches: HashMap<u16, usize> = tiles
        .iter()
        .flat_map(|tile| {
            tile.edges
                .iter()
                .map(|&edge| (edge, edge.reverse_bits() >> 6))
        })
        .fold(HashMap::new(), |mut map, (original, reversed)| {
            *map.entry(original).or_default() += 1;
            *map.entry(reversed).or_default() += 1;
            map
        });

    // go through each tile, and for each edge check how many matches
    // want tiles that have no matches on 2 edges, these are the corners
    // an edge with no matches will have a value of 1 in the map, itself
    // then get the product of the ids
    tiles
        .iter()
        .filter(|tile| {
            tile.edges
                .iter()
                .filter(|edge| edge_matches[edge] > 1)
                .count()
                == 2
        })
        .map(|tile| tile.id)
        .product::<usize>()
}
