use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/006.txt");

    let start = Instant::now();
    println!("Part one: {}", part_one_old(input));
    println!("Time taken: {:#?}", start.elapsed());

    let start = Instant::now();
    println!("Part one bits: {}", part_one_bits(input));
    println!("Time taken: {:#?}", start.elapsed());

    let start = Instant::now();
    println!("Part two: {}", part_two(input));
    println!("Time taken: {:#?}", start.elapsed());
}

fn part_one_old(input: &str) -> usize {
    input
        // split iterator over every blank line
        .split("\n\n")
        // remove newlines from groups responses then collect the characters into an array
        .map(|group| group.replace('\n', "").chars().collect::<Vec<char>>())
        // accumulate the number of unique responses and return it
        .fold(0, |sum, mut responses| {
            // sort the vector so we can remove duplicates
            responses.sort_unstable();
            // removes duplicate consecutive values (hence must be sorted)
            responses.dedup();
            // add the number of unique responses to the sum
            sum + responses.len()
        })
}

fn part_one_bits(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                // get the ascii byte for each character
                .bytes()
                .filter(|b| b != &b'\n')
                // starting with 32 bits with all 0, set the bit at the location for that character in my map
                // e.g. cba | 101 would mean an a response and a response
                .fold(0_u32, |responses, byte| responses | 1 << (byte - b'a'))
                // count the number of 1 bits (very helpful method)
                .count_ones()
        })
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                // not concatenating to the same line anymore
                .lines()
                // for each line now get all the responses, still as bit mapping
                .map(|line| {
                    line.bytes()
                        .fold(0_u32, |responses, byte| responses | 1 << (byte - b'a'))
                })
                // starting with all 1's, and'ing the accumulator will show which responses were true for all lines
                .fold(4_294_967_295_u32, |group, byte| group & byte)
                // count all the 1s
                .count_ones()
        })
        .sum()
}
