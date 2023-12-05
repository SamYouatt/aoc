use regex::Regex;

#[derive(Debug)]
struct MapRange {
    start: isize,
    end: isize,
    offset: isize,
}

impl MapRange {
    fn parse(mapping: &str) -> MapRange {
        let mut numbers = mapping.split_whitespace();

        let dest_start = numbers
            .next()
            .unwrap()
            .parse::<isize>()
            .expect("Failed to parse destination start");
        let source_start = numbers
            .next()
            .unwrap()
            .parse::<isize>()
            .expect("Failed to parse source start");
        let length = numbers
            .next()
            .unwrap()
            .parse::<isize>()
            .expect("Failed to parse length");

        let start = source_start;
        let end = source_start + length;
        let offset = dest_start - source_start;

        MapRange { start, end, offset }
    }

    fn try_map(&self, source: isize) -> Option<isize> {
        if source >= self.start && source < self.end {
            return Some(source + self.offset);
        }

        None
    }
}

#[derive(Debug)]
struct Mapping {
    mappings: Vec<MapRange>,
}

impl Mapping {
    fn parse(group: &str) -> Mapping {
        let mappings = group
            .lines()
            .skip(1)
            .map(|mapping| MapRange::parse(mapping))
            .collect();

        Mapping { mappings }
    }

    fn process(&self, source: isize) -> isize {
        for mapping in &self.mappings {
            if let Some(mapped_source) = mapping.try_map(source) {
                return mapped_source;
            }
        }

        return source;
    }
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {}", answer1);
}

fn part_1(input: &str) -> isize {
    let match_digits = Regex::new(r"\d+").expect("Failed to compile regex");

    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seeds: Vec<isize> = match_digits
        .find_iter(seeds)
        .map(|seed| {
            seed.as_str()
                .parse::<isize>()
                .expect("Failed to parse number")
        })
        .collect();

    let mappings: Vec<Mapping> = rest
        .split("\n\n")
        .map(|group| Mapping::parse(group))
        .collect();

    seeds
        .iter()
        .map(|seed| {
            let mut mapped_seed = *seed;
            for mapping in &mappings {
                mapped_seed = mapping.process(mapped_seed);
            }
            mapped_seed
        })
        .min()
        .unwrap()
}
