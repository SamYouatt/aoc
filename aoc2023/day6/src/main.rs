use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");

    let answer2_by_roots = part_2_alt(input);
    println!("Part 2 via quadratic roots: {answer2_by_roots}");
}

fn part_1(input: &str) -> usize {
    let match_digit = Regex::new(r"\d+").expect("failed to compile regex");

    let (times, records) = input.split_once('\n').unwrap();

    let times = match_digit
        .find_iter(times)
        .map(|time| time.as_str().parse::<usize>().unwrap());
    let records = match_digit
        .find_iter(records)
        .map(|record| record.as_str().parse::<usize>().unwrap());

    let record_distances: HashMap<_, _> = times.zip(records).collect();

    record_distances
        .iter()
        .map(|(time, rec)| {
            (1..*time)
                .map(|btn_time| (time - btn_time) * btn_time)
                .filter(|dist| dist > &rec)
                .count()
        })
        .product()
}

fn part_2(input: &str) -> usize {
    let (time, record) = input.split_once('\n').unwrap();

    fn parse_line(line: &str) -> usize {
        line.split_once(':')
            .unwrap()
            .1
            .trim()
            .replace(" ", "")
            .parse()
            .unwrap()
    }

    let time = parse_line(time);
    let record = parse_line(record);

    (1..time)
        .map(|btn_time| (time - btn_time) * btn_time)
        .filter(|dist| dist > &record)
        .count()
}

fn part_2_alt(input: &str) -> usize {
    let (time, record) = input.split_once('\n').unwrap();

    fn parse_line(line: &str) -> usize {
        line.split_once(':')
            .unwrap()
            .1
            .trim()
            .replace(" ", "")
            .parse()
            .unwrap()
    }

    let time = parse_line(time);
    let record = parse_line(record);

    let (root1, root2) = find_roots(time, record);
    integers_between(root1, root2)
}

fn find_roots(time: usize, distance: usize) -> (f64, f64) {
    let time = time as f64;
    let distance = distance as f64;

    let root_1 = (time + (time * time - 4.0 * distance).sqrt()) / 2.0;
    let root_2 = (time - (time * time - 4.0 * distance).sqrt()) / 2.0;

    (root_1, root_2)
}

fn integers_between(first: f64, second: f64) -> usize {
    (second.floor() - first.floor()).abs() as usize
}
