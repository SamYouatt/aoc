#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|x| x.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input
        .windows(2)
        .filter(|pair| pair[1] > pair[0])
        .count()
        .try_into()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let mut count = 0;
    for i in 3..input.len() {
        if input[i - 3] < input[i] {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const input: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input), 5);
    }
}
