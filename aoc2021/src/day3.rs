#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let num_bits = input.lines().next().unwrap().len();
    (
        num_bits,
        input
            .lines()
            // need to convert using string radix instead of parsing
            .map(|line| usize::from_str_radix(line, 2).unwrap())
            .collect(),
    )
}

#[aoc(day3, part1)]
fn solve_part1(input: &(usize, Vec<usize>)) -> usize {
    let num_bits = input.0;
    let codes = &input.1;
    let mut gamma = 0;

    (0..num_bits).for_each(|i| {
        let num_appearances = codes.iter().filter(|&&code| (code >> i) & 1 == 1).count();
        // if it appears in over half
        if num_appearances > (codes.len() >> 1) {
            gamma += 1 << i;
        } else {
            gamma += 0 << i;
        }
    });

    let epsilon = gamma ^ ((1 << num_bits) - 1);

    gamma * epsilon
}

#[aoc(day3, part2)]
fn solve_part2(input: &(usize, Vec<usize>)) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        let input = parse_input(INPUT);
        assert_eq!(solve_part1(&input), 198);
    }
}
