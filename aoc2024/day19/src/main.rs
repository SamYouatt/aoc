use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(PartialEq, Eq, Hash)]
enum Towel {
    Red,
    White,
    Green,
    Ublue,
    Black,
}

impl Towel {
    fn from_byte(byte: u8) -> Towel {
        match byte {
            b'r' => Towel::Red,
            b'u' => Towel::Ublue,
            b'g' => Towel::Green,
            b'b' => Towel::Black,
            b'w' => Towel::White,
            _ => panic!("bad towel"),
        }
    }
}

fn part_1(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels
        .split(", ")
        .map(|t| t.bytes().map(Towel::from_byte).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let designs = designs
        .lines()
        .map(|d| d.trim().bytes().map(Towel::from_byte).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|d| all_possible(&d, &towels, &mut cache) > 0)
        .count()
}

fn part_2(input: &str) -> usize {
    let (towels, designs) = input.split_once("\n\n").unwrap();

    let towels = towels
        .split(", ")
        .map(|t| t.bytes().map(Towel::from_byte).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let designs = designs
        .lines()
        .map(|d| d.trim().bytes().map(Towel::from_byte).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    designs
        .iter()
        .fold(0, |acc, d| acc + all_possible(&d, &towels, &mut cache))
}

fn all_possible<'a>(
    design: &'a [Towel],
    towels: &Vec<Vec<Towel>>,
    cache: &mut HashMap<&'a [Towel], usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(n) = cache.get(&design) {
        return *n;
    }

    let mut ways = 0;
    for t in towels {
        if design.starts_with(t) {
            ways += all_possible(&design[t.len()..], towels, cache);
        }
    }

    cache.insert(&design, ways);
    ways
}
