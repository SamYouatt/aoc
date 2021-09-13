use std::{
    collections::{HashMap, LinkedList},
    time::Instant,
};

struct TileEdges {
    id: usize,
    edges: [u16; 4],
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    grid: Vec<Vec<u8>>,
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
    let mut tiles: Vec<Tile> = input
        .split("\n\n")
        .map(|chunk| {
            let (id, tile) = chunk.split_once('\n').unwrap();
            let id: usize = id[5..].trim_end_matches(':').parse().unwrap();

            let pixels: Vec<Vec<u8>> = tile
                .split('\n')
                .map(|line| line.bytes().collect())
                .collect();

            Tile { id, grid: pixels }
        })
        .collect();

    println!("{}", tiles.len());

    let mut picture = HashMap::<Coord, Tile>::new();
    let mut queue = LinkedList::<Coord>::new();

    picture.insert(Coord::from(0, 0), tiles.pop().unwrap());
    queue.push_back(Coord::from(0, 0));

    while let Some(position) = queue.pop_front() {
        let mut remove = vec![];

        tiles.iter().enumerate().for_each(|(i, tile)| {
            let mut pixels = tile.grid.clone();
            'variations: for _flip in 0..=1 {
                for _rotation in 0..4 {
                    // see if it fits and if it does add it to the picture at the appropriate relative coords
                    if let Some(offset) = find_fit(&picture[&position].grid, &pixels) {
                        let new_position =
                            Coord::from(position.x + offset.x, position.y + offset.y);

                        if let std::collections::hash_map::Entry::Vacant(e) =
                            picture.entry(new_position)
                        {
                            remove.push(i);
                            e.insert(tile.clone());
                            queue.push_back(new_position);
                            break 'variations;
                        }
                    }
                    pixels = rotate(pixels);
                }
                pixels = flip(pixels);
            }
        });

        remove.iter().for_each(|id| {
            tiles.remove(*id);
        });
    }

    let monster: Vec<Vec<u8>> = include_str!("../../inputs/monster.txt")
        .split('\n')
        .map(|line| line.bytes().collect())
        .collect();

    let mut full_picture = Vec::with_capacity(80);

    let min_x = picture
        .iter()
        .min_by_key(|element| element.0.x)
        .unwrap()
        .0
        .x;
    let max_x = picture
        .iter()
        .max_by_key(|element| element.0.x)
        .unwrap()
        .0
        .x;
    let min_y = picture
        .iter()
        .min_by_key(|element| element.0.y)
        .unwrap()
        .0
        .y;
    let max_y = picture
        .iter()
        .max_by_key(|element| element.0.y)
        .unwrap()
        .0
        .y;

    println!("{} {}", max_x, min_x);

    let mut seamonster_location = vec![vec![false; 80]; 80];
    let mut x = 0;
    for chunk_x in min_x..=max_x {
        for tile_x in 1..=8 {
            // do something
            full_picture.push(Vec::with_capacity(80));
            for chunk_y in min_y..=max_y {
                for tile_y in 1..=8 {
                    println!("{} {}", chunk_x, chunk_y);
                    println!("{}", picture.len());
                    println!("{:?}", picture.keys());
                    full_picture[x]
                        .push(picture[&Coord::from(chunk_x, chunk_y)].grid[tile_x][tile_y])
                }
            }
            x += 1;
        }
    }

    for _ in 0..=1 {
        for _ in 0..4 {
            for x in 0..full_picture.len() - monster.len() + 1 {
                for y in 0..full_picture[0].len() - monster[0].len() + 1 {
                    let mut is_monster = true;
                    'check_monster: for delta_x in 0..monster.len() {
                        for delta_y in 0..monster[0].len() {
                            if monster[delta_x][delta_y] == b'#'
                                && full_picture[x + delta_x][y + delta_y] != b'#'
                            {
                                is_monster = false;
                                break 'check_monster;
                            }
                        }
                    }

                    if is_monster {
                        for delta_x in 0..monster.len() {
                            for delta_y in 0..monster[0].len() {
                                if monster[delta_x][delta_y] == b'#' {
                                    seamonster_location[x + delta_x][y + delta_y] = true;
                                }
                            }
                        }
                    }
                }
            }
            seamonster_location = rotate(seamonster_location);
            full_picture = rotate(full_picture);
        }
        seamonster_location = flip(seamonster_location);
        full_picture = flip(full_picture);
    }

    let mut non_sea_monster_points = 0;
    for x in 0..full_picture.len() {
        for y in 0..full_picture[0].len() {
            non_sea_monster_points +=
                (full_picture[x][y] == b'#' && !seamonster_location[x][y]) as usize;
        }
    }

    println!("{}", non_sea_monster_points);
    0
}

fn find_fit(grid1: &[Vec<u8>], grid2: &[Vec<u8>]) -> Option<Coord> {
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
