use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");
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
            let all_distances: Vec<usize> = (1..*time)
                .map(|btn_time| {
                    let remaining = time - btn_time;
                    remaining * btn_time
                })
                .collect();

            all_distances.iter().filter(|dist| dist > &rec).count()
        })
        .product()
}
