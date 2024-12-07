fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
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
            can_work(nums.clone(), Operator::Plus, *result)
                || can_work(nums.to_vec(), Operator::Mult, *result)
        })
        .fold(0, |acc, (result, _)| acc + result)
}

enum Operator {
    Plus,
    Mult,
}

fn can_work(nums: Vec<usize>, operator: Operator, desired: usize) -> bool {
    if nums.len() == 1 && nums[0] == desired {
        return true;
    }

    if nums.len() == 1 && nums[0] != desired {
        return false;
    }

    let head = match operator {
        Operator::Plus => nums[0] + nums[1],
        Operator::Mult => nums[0] * nums[1],
    };

    let mut new_nums = vec![head];
    new_nums.extend_from_slice(&nums[2..]);

    return can_work(new_nums.clone(), Operator::Plus, desired)
        || can_work(new_nums, Operator::Mult, desired);
}
