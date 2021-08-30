use std::{collections::HashMap, time::Instant};

fn main() {
    let mut numbers: Vec<usize> = include_str!("../../inputs/010.txt")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    let start = Instant::now();
    let answer = get_split(&numbers);

    println!("Part one: {}", answer.0 * answer.1);
    println!("Time taken: {:#?}", start.elapsed());

    let start = Instant::now();
    let answer = find_routes(&numbers);
    println!("Part two: {}", answer);
    println!("Time taken: {:#?}", start.elapsed());
}

fn get_split(numbers: &[usize]) -> (usize, usize) {
    let (mut num_ones, mut num_threes) = (0, 0);

    numbers.windows(2).for_each(|window| {
        if window[1] - window[0] == 3 {
            num_threes += 1;
        } else if window[1] - window[0] == 1 {
            num_ones += 1;
        }
    });

    (num_ones, num_threes)
}

fn find_routes(numbers: &[usize]) -> usize {
    //let mut sum = 0;

    let mut paths = HashMap::new();

    paths.insert(numbers.last().copied().unwrap(), 1);

    for i in (0..(numbers.len() - 1)).into_iter().rev() {
        let neighbours = ((i + 1)..=std::cmp::min(i + 3, numbers.len() - 1))
            .into_iter()
            .filter_map(|j| {
                if (1..=3).contains(&(numbers[j] - numbers[i])) {
                    Some(paths.get(&numbers[j]).unwrap())
                } else {
                    None
                }
            })
            .sum();
        paths.insert(numbers[i], neighbours);
    }

    *paths.get(&0).unwrap()
}

#[test]
fn test_get_split() {
    let mut numbers: Vec<usize> = "33\n28\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3"
    .lines()
    .map(|line| line.parse().unwrap())
    .collect();

    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    let split = get_split(&numbers);
    assert_eq!(split.0, 22);
    assert_eq!(split.1, 10);
}

#[test]
fn test_number_routes() {
    let mut numbers: Vec<usize> = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4"
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + 3);

    let routes = find_routes(&numbers);
    assert_eq!(routes, 8);
}
