use std::time::Instant;

fn main() {
    let numbers: Vec<usize> = include_str!("../../inputs/009.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let start = Instant::now();
    let answer = numbers
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
        .unwrap()[25];

    println!("Part one: {:#?}", answer);
    println!("Time taken: {:#?}", start.elapsed());
}
