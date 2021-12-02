use std::time::Instant;

fn main() {
    let numbers: Vec<usize> = include_str!("../../inputs/009.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let start = Instant::now();
    let answer = part_one(&numbers);
    println!("Part one: {}", answer);
    println!("Time taken: {:#?}", start.elapsed());

    let start = Instant::now();
    println!("Part two: {}", part_two(&numbers, answer));
    println!("Time taken: {:#?}", start.elapsed());
}

fn part_one(numbers: &[usize]) -> usize {
    numbers
        .windows(26)
        .find(|window| {
            for i in 0..24 {
                for j in (i + 1)..25 {
                    if window[i] + window[j] == window[25] {
                        return false;
                    }
                }
            }
            true
        })
        .unwrap()[25]
}

fn part_two(numbers: &[usize], goal: usize) -> usize {
    let mut left = 0;
    let mut right = 1;
    let mut acc = numbers[left] + numbers[right];

    while acc != goal {
        while acc < goal {
            right += 1;
            acc += numbers[right]
        }
        while acc > goal {
            acc -= numbers[left];
            left += 1;
        }
    }

    let range = &numbers[left..=right];

    range.iter().min().unwrap() + range.iter().max().unwrap()
}
