use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let equations = input
        .lines()
        .map(|line| {
            let (result, values) = line.split_once(": ").unwrap();
            let values = values
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (result.parse::<usize>().unwrap(), values)
        })
        .collect::<Vec<_>>();

    equations
        .into_iter()
        .filter(|(result, nums)| solvable(*result, &nums, nums.len() - 1, false))
        .fold(0, |acc, (result, _)| acc + result)
}

fn part_2(input: &str) -> usize {
    let equations = input
        .lines()
        .map(|line| {
            let (result, values) = line.split_once(": ").unwrap();
            let values = values
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            (result.parse::<usize>().unwrap(), values)
        })
        .collect::<Vec<_>>();

    equations
        .par_iter()
        .filter(|(result, nums)| solvable(*result, &nums, nums.len() - 1, true))
        .map(|(result, _)| *result)
        .reduce(|| 0, |acc, result| acc + result)
}

fn solvable(remainder: usize, numbers: &[usize], tail_ptr: usize, allow_concat: bool) -> bool {
    if tail_ptr == 0 {
        return remainder == numbers[0];
    }

    if remainder % numbers[tail_ptr] == 0
        && solvable(
            remainder / numbers[tail_ptr],
            numbers,
            tail_ptr - 1,
            allow_concat,
        )
    {
        return true;
    }

    if remainder >= numbers[tail_ptr]
        && solvable(
            remainder - numbers[tail_ptr],
            numbers,
            tail_ptr - 1,
            allow_concat,
        )
    {
        return true;
    }

    let remainder_str = remainder.to_string();
    let tail_num_str = numbers[tail_ptr].to_string();
    if allow_concat
        && remainder > numbers[tail_ptr]
        && remainder_str.ends_with(&tail_num_str)
        && solvable(
            remainder_str[0..(remainder_str.len() - tail_num_str.len())]
                .parse::<usize>()
                .unwrap(),
            numbers,
            tail_ptr - 1,
            allow_concat,
        )
    {
        return true;
    }

    false
}
