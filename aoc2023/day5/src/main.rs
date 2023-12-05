use std::{cmp::min, ops::Range};

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
        let mut mappings = group
            .lines()
            .skip(1)
            .map(|mapping| MapRange::parse(mapping))
            .collect::<Vec<MapRange>>();

        mappings.sort_by(|a, b| a.start.cmp(&b.start));

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

    fn mapped_ranges(&self, source_range: Range<isize>) -> Vec<Range<isize>> {
        let mut new_ranges = vec![];
        let mut current_start = source_range.start;

        for mapping in self
            .mappings
            .iter()
            .skip_while(|mapping| mapping.end < source_range.start)
        {
            if current_start < mapping.start {
                let new_range = current_start..min(source_range.end, mapping.start);
                new_ranges.push(new_range);
                current_start = mapping.start;
            }

            if current_start >= mapping.end {
                break;
            }

            let offset_start = current_start + mapping.offset;
            let offset_end = min(source_range.end, mapping.end) + mapping.offset;
            let new_range = offset_start..offset_end;
            new_ranges.push(new_range);

            current_start = mapping.end;

            if current_start >= source_range.end {
                break;
            }
        }

        if current_start < source_range.end {
            let new_range = current_start..source_range.end;
            new_ranges.push(new_range);
        }

        new_ranges
    }
}

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {}", answer1);

    let answer2 = part_2(input);
    println!("Part 2: {}", answer2);
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

fn part_2(input: &str) -> isize {
    let match_digits = Regex::new(r"\d+").expect("Failed to compile regex");

    let (seeds, rest) = input.split_once("\n\n").unwrap();
    let seed_ranges: Vec<Range<isize>> = match_digits
        .find_iter(seeds)
        .map(|seed| {
            seed.as_str()
                .parse::<isize>()
                .expect("Failed to parse number")
        })
        .collect::<Vec<isize>>()
        .chunks_exact(2)
        .map(|chunk| chunk[0]..(chunk[0] + chunk[1]))
        .collect();

    let mappings: Vec<Mapping> = rest
        .split("\n\n")
        .map(|group| Mapping::parse(group))
        .collect();

    let mut mapped_ranges: Vec<Range<isize>> = vec![];

    for seed_range in seed_ranges {
        let mut current_ranges = vec![seed_range];
        for mapping in &mappings {
            let new_ranges = current_ranges
                .iter()
                .flat_map(|range| mapping.mapped_ranges(range.clone()))
                .collect();
            current_ranges = new_ranges;
        }
        mapped_ranges.append(&mut current_ranges);
    }

    mapped_ranges
        .iter()
        .min_by(|a, b| a.start.cmp(&b.start))
        .unwrap()
        .start
}
