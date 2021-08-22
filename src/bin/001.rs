use std::{fs, path::Path};

fn main() {
    let mut numbers: [i32; 200] = [0; 200];

    let input = fs::read_to_string(Path::new("/home/sam/Documents/code/aoc2020/inputs/001.txt"))
        .expect("something went wrong");

    input
        .lines()
        .enumerate()
        .for_each(|(i, x)| numbers[i] = x.parse().unwrap());

    numbers.sort();

    let (a, b) = find_pair(&numbers);
    println!("a: {}, b: {}, ab: {}", a, b, a * b);

    let (c, d, e) = find_triple(&numbers);
    println!("a: {}, b: {}, c: {}, abc: {}", c, d, e, c * d * e);
}

fn find_pair(numbers: &[i32]) -> (i32, i32) {
    for i in 0..numbers.len() - 1 {
        for j in 1..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return (numbers[i], numbers[j]);
            }
        }
    }
    return (0, 0);
}

fn find_triple(numbers: &[i32]) -> (i32, i32, i32) {
    for i in 0..numbers.len() - 2 {
        for j in 1..numbers.len() - 1 {
            for k in 2..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return (numbers[i], numbers[j], numbers[k]);
                }
            }
        }
    }
    return (0, 0, 0);
}

#[test]
fn test_find_pair_on_question() {
    let mut numbers = [1721, 979, 366, 299, 675, 1456];

    let (a, b) = find_pair(&numbers);

    assert_eq!(a * b, 514579)
}

#[test]
fn test_triple_on_question() {
    let mut numbers = [1721, 979, 366, 299, 675, 1456];

    let (a, b, c) = find_triple(&numbers);

    assert_eq!(a * b * c, 241861950)
}
