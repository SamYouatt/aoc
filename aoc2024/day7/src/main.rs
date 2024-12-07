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
        .filter(|(result, nums)| {
            can_work(nums[0], &nums[1..], Operator::Plus, *result, false)
                || can_work(nums[0], &nums[1..], Operator::Mult, *result, false)
        })
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
        .filter(|(result, nums)| {
            can_work(nums[0], &nums[1..], Operator::Plus, *result, true)
                || can_work(nums[0], &nums[1..], Operator::Mult, *result, true)
                || can_work(nums[0], &nums[1..], Operator::Concat, *result, true)
        })
        .map(|(result, _)| *result)
        .reduce(|| 0, |acc, result| acc + result)
}

enum Operator {
    Plus,
    Mult,
    Concat,
}

fn can_work(
    head: usize,
    tail: &[usize],
    operator: Operator,
    desired: usize,
    allow_concat: bool,
) -> bool {
    if head > desired {
        return false;
    }

    if tail.len() == 0 {
        return head == desired;
    }

    let new_head = match operator {
        Operator::Plus => head + tail[0],
        Operator::Mult => head * tail[0],
        Operator::Concat => format!("{}{}", head, tail[0]).parse::<usize>().unwrap(),
    };

    return can_work(new_head, &tail[1..], Operator::Plus, desired, allow_concat)
        || can_work(new_head, &tail[1..], Operator::Mult, desired, allow_concat)
        || (allow_concat
            && can_work(
                new_head,
                &tail[1..],
                Operator::Concat,
                desired,
                allow_concat,
            ));
}
