use std::{
    collections::{hash_map::Entry, HashMap, LinkedList},
    time::Instant,
};

use itertools::Itertools;

struct TileEdges {
    id: usize,
    edges: [u16; 4],
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<u8>>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn from(x: isize, y: isize) -> Coord {
        Coord { x, y }
    }
}

fn main() {
    let input = include_str!("../../inputs/020.txt");

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} in {:#?}", part_two(input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let tiles = input
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

            TileEdges {
                id,
                edges: [top_edge, left_edge, right_edge, bottom_edge],
            }
        })
        .collect::<Vec<TileEdges>>();

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

fn part_two(input: &str) -> usize {
    // chose to have the sea monster in its own file instead of trying to write it in by hand
    let sea_monster: Vec<Vec<u8>> = include_str!("../../inputs/monster.txt")
        .split('\n')
        .map(|line| line.bytes().collect())
        .collect();

    let mut tiles: Vec<Tile> = input
        .split("\n\n")
        .map(|chunk| {
            let (id, tile) = chunk.split_once('\n').unwrap();
            let id: usize = id[5..].trim_end_matches(':').parse().unwrap();

            let pixels: Vec<Vec<u8>> = tile
                .split('\n')
                .map(|line| line.bytes().collect())
                .collect();

            Tile { id, pixels }
        })
        .collect();

    // represents the positioned tiles, with the relative coord offset from the first placed
    // and the tile with the appropriate rotation and flips applied
    let mut arranged_tiles = HashMap::<Coord, Tile>::new();
    let mut queue = LinkedList::<Coord>::new();

    arranged_tiles.insert(Coord::from(0, 0), tiles.pop().unwrap());
    queue.push_back(Coord::from(0, 0));

    // from an offset coord, iterate through every tile and try and place it
    // with all rotation and flip variations
    while let Some(position) = queue.pop_front() {
        let mut ids_to_remove = vec![];

        tiles.iter().for_each(|tile| {
            let mut pixels = tile.pixels.clone();
            'variations: for _flip in 0..=1 {
                for _rotation in 0..4 {
                    if let Some(offset) = find_fit(&arranged_tiles[&position].pixels, &pixels) {
                        let new_position =
                            Coord::from(position.x + offset.x, position.y + offset.y);

                        if let Entry::Vacant(e) = arranged_tiles.entry(new_position) {
                            ids_to_remove.push(tile.id);
                            e.insert(Tile {
                                id: tile.id,
                                pixels,
                            });
                            queue.push_back(new_position);
                            break 'variations;
                        }
                    }
                    pixels = rotate(pixels);
                }
                pixels = flip(pixels);
            }
        });

        tiles.retain(|tile| !ids_to_remove.contains(&tile.id));
    }

    let min_x = arranged_tiles.iter().min_by_key(|a| a.0.x).unwrap().0.x;
    let max_x = arranged_tiles.iter().max_by_key(|a| a.0.x).unwrap().0.x;
    let min_y = arranged_tiles.iter().min_by_key(|a| a.0.y).unwrap().0.y;
    let max_y = arranged_tiles.iter().max_by_key(|a| a.0.y).unwrap().0.y;

    // picture is the arranged tiles with the borders trimmed
    let mut picture = Vec::with_capacity(96);
    let mut sea_monster_points = vec![vec![false; 96]; 96];
    // go through the arranged tiles in order and trim the borders and create new picture
    let mut i = 0;
    for chunk_y in min_y..=max_y {
        for tile_y in 1..9 {
            picture.push(Vec::with_capacity(96));
            for chunk_x in min_x..=max_x {
                for tile_x in 1..9 {
                    picture[i]
                        .push(arranged_tiles[&Coord::from(chunk_x, chunk_y)].pixels[tile_y][tile_x])
                }
            }
            i += 1;
        }
    }

    // for each coordinate of the image, try and place the sea monster in all its variations
    for _flip in 0..=1 {
        for _rotation in 0..4 {
            (0..picture[0].len() - sea_monster[0].len() + 1)
                .cartesian_product(0..picture.len() - sea_monster.len() + 1)
                .for_each(|(x, y)| {
                    // check if the sea monster will fit in that location
                    let mut is_monster = true;
                    'check_monster: for delta_x in 0..sea_monster[0].len() {
                        for delta_y in 0..sea_monster.len() {
                            if sea_monster[delta_y][delta_x] == b'#'
                                && picture[y + delta_y][x + delta_x] != b'#'
                            {
                                is_monster = false;
                                break 'check_monster;
                            }
                        }
                    }

                    // if it does fit then add it to the list of sea monster points
                    if is_monster {
                        (0..sea_monster[0].len())
                            .cartesian_product(0..sea_monster.len())
                            .for_each(|(dx, dy)| {
                                if sea_monster[dy][dx] == b'#' {
                                    sea_monster_points[y + dy][x + dx] = true;
                                }
                            });
                    }
                });
            sea_monster_points = rotate(sea_monster_points);
            picture = rotate(picture);
        }
        sea_monster_points = flip(sea_monster_points);
        picture = flip(picture);
    }

    // go through the image and count the number of points that are not sea monster points
    (0..picture[0].len())
        .cartesian_product(0..picture.len())
        .fold(0, |non_monster, (x, y)| {
            non_monster + (picture[y][x] == b'#' && !sea_monster_points[y][x]) as usize
        })
}

fn find_fit(grid1: &[Vec<u8>], grid2: &[Vec<u8>]) -> Option<Coord> {
    // note that the y coordinates have positive as down and negative as up
    // this is because when we work with the vectors the y starts at 0 and goes up
    if grid1.first() == grid2.last() {
        // above
        Some(Coord::from(0, -1))
    } else if grid1.last() == grid2.first() {
        // below
        Some(Coord::from(0, 1))
    } else {
        let mut on_left = true;
        let mut on_right = true;

        for x in 0..grid1.len() {
            on_left &= grid1[x].first() == grid2[x].last();
            on_right &= grid1[x].last() == grid2[x].first();
        }

        if on_left {
            Some(Coord::from(-1, 0))
        } else if on_right {
            Some(Coord::from(1, 0))
        } else {
            None
        }
    }
}

fn flip<T>(grid: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let original_size = (grid.len(), grid[0].len());

    let mut new_grid: Vec<Vec<T>> = Vec::with_capacity(original_size.0);
    for x in 0..original_size.0 {
        new_grid.push(Vec::with_capacity(original_size.1));
        for y in 0..original_size.1 {
            new_grid[x].push(grid[original_size.0 - x - 1][y]);
        }
    }

    new_grid
}

fn rotate<T>(grid: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let original_size = (grid.len(), grid[0].len());

    let mut new_grid: Vec<Vec<T>> = Vec::with_capacity(original_size.1);
    for x in 0..original_size.1 {
        new_grid.push(Vec::with_capacity(original_size.0));
        for y in 0..original_size.0 {
            new_grid[x].push(grid[original_size.1 - y - 1][x]);
        }
    }

    new_grid
}
