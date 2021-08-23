use std::{fs, path::Path, time::Instant};

fn main() {
    let mut numbers: [i32; 200] = [0; 200];

    let input = fs::read_to_string(Path::new("/home/sam/Documents/code/aoc2020/inputs/001.txt"))
        .expect("something went wrong");

    input
        .lines()
        .enumerate()
        .for_each(|(i, x)| numbers[i] = x.parse().unwrap());

    numbers.sort();

    // Part 1
    let start = Instant::now();
    let (a, b) = find_pair(&numbers);
    let duration = start.elapsed();
    println!("a: {}, b: {}, ab: {}", a, b, a * b);
    println!("Time taken: {:?}", duration);

    // Part 2
    let start = Instant::now();
    let (a, b, c) = find_triple(&numbers);
    let duration = start.elapsed();
    println!("a: {}, b: {}, c: {}, abc: {}", a, b, c, a * b * c);
    println!("Time taken: {:?}", duration);
}

fn find_pair(numbers: &[i32]) -> (i32, i32) {
    // start with a left pointer and right pointer
    // add the two values together,
    // if too big -> reduce right pointer,
    // if too small -> increase left pointer

    let mut left = 0;
    let mut right = numbers.len() - 1;

    while left != right && numbers[left] + numbers[right] != 2020 {
        if numbers[left] + numbers[right] > 2020 {
            right -= 1;
        } else {
            left += 1;
        }
    }
    return (numbers[left], numbers[right]);
}

fn find_triple(numbers: &[i32]) -> (i32, i32, i32) {
    // using sliding window method again but with first pointer fixed at 0
    // two remaining pointer should add to 2020 - fixed
    // if all possibilities of left and right pointer tried, move fixed up by 1 and go again

    for fixed in 0..numbers.len() - 2 {
        let mut left = fixed + 1;
        let mut right = numbers.len() - 1;

        while left < right && right < numbers.len() {
            if numbers[left] + numbers[right] == 2020 - numbers[fixed] {
                return (numbers[fixed], numbers[left], numbers[right]);
            } else if numbers[left] + numbers[right] < 2020 - numbers[fixed] {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    return (0, 0, 0);
}

#[test]
fn test_find_pair_on_question() {
    let numbers = [1721, 979, 366, 299, 675, 1456];

    let (a, b) = find_pair(&numbers);

    assert_eq!(a * b, 514579)
}

#[test]
fn test_triple_on_question() {
    let numbers = [1721, 979, 366, 299, 675, 1456];

    let (a, b, c) = find_triple(&numbers);

    assert_eq!(a * b * c, 241861950)
}
