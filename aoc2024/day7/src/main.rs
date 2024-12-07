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
            can_work(nums.clone(), Operator::Plus, *result, false)
                || can_work(nums.to_vec(), Operator::Mult, *result, false)
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
        .into_iter()
        .filter(|(result, nums)| {
            can_work(nums.clone(), Operator::Plus, *result, true)
                || can_work(nums.to_vec(), Operator::Mult, *result, true)
                || can_work(nums.to_vec(), Operator::Concat, *result, true)
        })
        .fold(0, |acc, (result, _)| acc + result)
}

enum Operator {
    Plus,
    Mult,
    Concat,
}

fn can_work(nums: Vec<usize>, operator: Operator, desired: usize, allow_concat: bool) -> bool {
    if nums.len() == 1 && nums[0] == desired {
        return true;
    }

    if nums.len() == 1 && nums[0] != desired {
        return false;
    }

    let head = match operator {
        Operator::Plus => nums[0] + nums[1],
        Operator::Mult => nums[0] * nums[1],
        Operator::Concat => format!("{}{}", nums[0], nums[1]).parse::<usize>().unwrap(),
    };

    let mut new_nums = vec![head];
    new_nums.extend_from_slice(&nums[2..]);

    return can_work(new_nums.clone(), Operator::Plus, desired, allow_concat)
        || can_work(new_nums.clone(), Operator::Mult, desired, allow_concat)
        || (allow_concat && can_work(new_nums, Operator::Concat, desired, allow_concat));
}
