use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> isize {
    input
        .lines()
        .map(|l| l.parse::<isize>().unwrap())
        .map(|x| {
            let mut secret_num = x;

            for _ in 0..2000 {
                secret_num = prune(mix(secret_num, secret_num * 64));
                secret_num = prune(mix(secret_num, secret_num / 32));
                secret_num = prune(mix(secret_num, secret_num * 2048));
            }

            secret_num
        })
        .sum()
}

fn part_2(input: &str) -> isize {
    let mut scores = HashMap::new();

    for price in input.lines().map(|l| l.parse::<isize>().unwrap()) {
        let mut secret_num = price;
        let mut nums = Vec::new();

        for _ in 0..2000 {
            secret_num = prune(mix(secret_num, secret_num * 64));
            secret_num = prune(mix(secret_num, secret_num / 32));
            secret_num = prune(mix(secret_num, secret_num * 2048));

            nums.push(secret_num % 10);
        }

        let mut buy_results = HashMap::new();
        for (n5, n4, n3, n2, n1) in nums.iter().rev().tuple_windows() {
            let changes = (n2 - n1, n3 - n2, n4 - n3, n5 - n4);
            buy_results.insert(changes, n5);
        }

        for (changes, result) in buy_results {
            *scores.entry(changes).or_insert(0) += result;
        }
    }

    scores.into_values().max().unwrap()
}

fn mix(secret: isize, x: isize) -> isize {
    secret ^ x
}

fn prune(secret: isize) -> isize {
    secret.rem_euclid(16777216)
}
