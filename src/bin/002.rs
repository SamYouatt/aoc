use std::{fs, path::Path, time::Instant};

struct PasswordEntry {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordEntry {
    fn new(line: &str) -> PasswordEntry {
        let pieces: Vec<&str> = line.split(|c| c == '-' || c == ' ').collect();
        let min: usize = pieces[0].parse().unwrap();
        let max: usize = pieces[1].parse().unwrap();
        let letter = pieces[2].chars().next().unwrap();
        let password = pieces[3].to_string();

        PasswordEntry {
            min,
            max,
            letter,
            password,
        }
    }
}

fn main() {
    let input = fs::read_to_string(Path::new("/home/sam/Documents/code/aoc2020/inputs/002.txt"))
        .expect("something went wrong");

    let entries: Vec<PasswordEntry> = input.lines().map(|line| PasswordEntry::new(line)).collect();

    // part one
    println!("Part one");
    println!(
        "Count correct: {}",
        entries
            .iter()
            .filter(|entry| is_pass_correct_part_one(entry))
            .count()
    );

    // part two
    println!("\nPart two");
    println!(
        "Count correct: {}",
        entries
            .iter()
            .filter(|entry| is_pass_correct_part_two(entry))
            .count()
    )
}

fn is_pass_correct_part_one(entry: &PasswordEntry) -> bool {
    let mut count = 0;

    entry.password.chars().for_each(|c| {
        if c == entry.letter {
            count += 1;
            if count > entry.max {
                return;
            }
        }
    });

    return count >= entry.min && count <= entry.max;
}

fn is_pass_correct_part_two(entry: &PasswordEntry) -> bool {
    (entry.password.chars().nth(entry.min - 1).unwrap() == entry.letter)
        ^ (entry.password.chars().nth(entry.max - 1).unwrap() == entry.letter)
}

#[test]
fn test_correct_password() {
    let min = 1;
    let max = 3;
    let letter = 'a';
    let password = "abcde".to_string();

    assert_eq!(
        is_pass_correct_part_one(&PasswordEntry {
            min,
            max,
            letter,
            password
        }),
        true
    );
}

#[test]
fn test_incorrect_password() {
    let min = 1;
    let max = 3;
    let letter = 'b';
    let password = "cdefg".to_string();

    assert_eq!(
        is_pass_correct_part_one(&PasswordEntry {
            min,
            max,
            letter,
            password
        }),
        false
    );
}
