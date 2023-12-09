fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

fn part_1(input: &str) -> isize {
    let sequences: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect();

    sequences.iter().map(|seq| geometric_prediction(seq)).sum()
}

fn part_2(input: &str) -> isize {
    let sequences: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .rev()
                .collect()
        })
        .collect();

    sequences.iter().map(|seq| geometric_prediction(seq)).sum()
}

fn geometric_prediction(sequence: &Vec<isize>) -> isize {
    if sequence.iter().all(|x| x == &0) {
        return 0;
    }

    let next_value = geometric_prediction(&find_differences(sequence));

    sequence.last().unwrap() + next_value
}

fn find_differences(sequence: &Vec<isize>) -> Vec<isize> {
    sequence.windows(2).map(|pair| pair[1] - pair[0]).collect()
}
