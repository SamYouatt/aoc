use std::fs;

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)

fn main() {
    let input = fs::read_to_string("/home/sam/Documents/code/aoc2020/inputs/004.txt")
        .expect("something went wrong");

    part_one(&input);
}

fn part_one(input: &str) {
    let passport_entries = get_passports(input);

    let valid_count = count_if_has_all_fields(&passport_entries);

    println!("{:?}", valid_count);
}

fn count_if_has_all_fields(passport_entries: &Vec<String>) -> usize {
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
        .count()
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
fn test_on_question() {
    let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";

    let passport_entries = get_passports(input);

    let valid_count = count_if_has_all_fields(&passport_entries);

    assert_eq!(valid_count, 2);
}
