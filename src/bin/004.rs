use std::{collections::HashMap, fs, time::Instant};

use regex::Regex;

fn main() {
    let start = Instant::now();
    part_one();
    let mut duration = start.elapsed();
    println!("Time taken: {:#?}", duration);
    part_two();
    duration = start.elapsed() - duration;
    println!("Time taken: {:#?}", duration)
}

fn part_one() {
    let required_fields: Vec<&str> = vec![
        "byr".into(),
        "iyr".into(),
        "eyr".into(),
        "hgt".into(),
        "hcl".into(),
        "ecl".into(),
        "pid".into(),
    ];

    println!(
        "Part one: {}",
        // includes a utf8 encoded file as a &str
        include_str!("../../inputs/004.txt")
            // create an iterator where each element is split based on blank line
            .split("\n\n")
            // turn each passport entry into a hash map
            .map(|entry| entry
                // take each element of iterator (passport entry) and split it through whitespace (handles multiple spaces etc.)
                .split_ascii_whitespace()
                // splits a string on the first appearance of the characeter, returns an iterator with elements of before and after arg
                // creates the field (before the :) and the value (after the :)
                .map(|field| field.split_once(':').unwrap())
                // collect the two values into a hash map
                .collect::<HashMap<&str, &str>>())
            // remove passports that don't contain all fields
            .filter(|passport| required_fields
                .iter()
                // returns only those which the closure holds true for every single element
                .all(|field| passport.contains_key(field)))
            // count how many valid passports there are
            .count()
    );
}

fn part_two() {
    let required_fields: Vec<&str> = vec![
        "byr".into(),
        "iyr".into(),
        "eyr".into(),
        "hgt".into(),
        "hcl".into(),
        "ecl".into(),
        "pid".into(),
    ];

    println!(
        "Part two: {}",
        // includes a utf8 encoded file as a &str
        include_str!("../../inputs/004.txt")
            // create an iterator where each element is split based on blank line
            .split("\n\n")
            // turn each passport entry into a hash map
            .map(|entry| entry
                // take each element of iterator (passport entry) and split it through whitespace (handles multiple spaces etc.)
                .split_ascii_whitespace()
                // splits a string on the first appearance of the characeter, returns an iterator with elements of before and after arg
                // creates the field (before the :) and the value (after the :)
                .map(|field| field.split_once(':').unwrap())
                // collect the two values into a hash map
                .collect::<HashMap<&str, &str>>())
            // remove passports that don't contain all fields
            .filter(|passport| required_fields
                .iter()
                // returns only those which the closure holds true for every single element
                .all(|field| passport.contains_key(field)))
            // remove passports that don't pass all the validation functions
            .filter(|passport| passport
                .iter()
                // the predicate this time is a method which validates based on what type of field it is given
                .all(|(field, value)| validate(field, value)))
            // count how many valid passports there are
            .count()
    );
}

/// Validates the value against the validation based on the field
fn validate(field: &str, value: &str) -> bool {
    match field {
        // byr - four digits at least 1920 and most 2002
        "byr" => (1920..=2002).contains(&value.parse().unwrap()),
        // iyr - four digits at least 2010 and at most 2020
        "iyr" => (2010..=2020).contains(&value.parse().unwrap()),
        // eyr - four digits at least 2020 and at most 2030
        "eyr" => (2020..=2030).contains(&value.parse().unwrap()),
        // hgt - number followed by cm or in
        //     - if cm then 150 <= num <= 193
        //     - if in then 49 <= num <= 76
        "hgt" => {
            if value.ends_with("cm") && value.len() == 5 {
                (150..=193).contains(&value[0..3].parse().unwrap())
            } else if value.ends_with("in") && value.len() == 4 {
                (49..=76).contains(&value[0..2].parse().unwrap())
            } else {
                false
            }
        }
        // hcl - a # followed by six character 0-9 or a-f
        "hcl" => {
            let regex = Regex::new(r"^#[0-9a-f]+$").unwrap();
            regex.is_match(value)
        }
        // ecl - one of amb, blu, brn, gry, grn, hzl, oth
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
        // pid - nine digit number, including leading zeroes
        "pid" => value.len() == 9,
        // cid is optional so can set it to always return true
        "cid" => true,
        _ => panic!(),
    }
}

fn get_passports_with_all_fields(passport_entries: &Vec<String>) -> Vec<&String> {
    let required_fields: Vec<String> = vec![
        "byr".into(),
        "iyr".into(),
        "eyr".into(),
        "hgt".into(),
        "hcl".into(),
        "ecl".into(),
        "pid".into(),
    ];

    passport_entries
        .iter()
        .filter(|line| required_fields.iter().all(|field| line.contains(field)))
        .collect()
}

fn get_passports(input: &str) -> Vec<String> {
    let lines = input.lines();

    let mut passport_entries = Vec::new();
    let mut acc = "".to_string();

    for line in lines {
        if line == "" {
            passport_entries.push(acc);
            acc = "".to_string();
        } else if acc == "" {
            acc = line.to_string();
        } else {
            acc = format!("{} {}", acc, line);
        }
    }
    passport_entries.push(acc);

    passport_entries
}

struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

#[test]
fn test_on_question_part_one() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";

    let passport_entries = get_passports(input);

    let valid_count = get_passports_with_all_fields(&passport_entries).len();

    assert_eq!(valid_count, 2);
}
