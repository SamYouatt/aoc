use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
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
