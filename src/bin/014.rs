use std::{collections::HashMap, time::Instant};

use regex::Regex;

struct Mask {
    set: usize,
    clear: usize,
}

fn main() {
    let input = include_str!("../../inputs/014.txt");

    let start = Instant::now();
    println!("Part one: {} in {:#?}", part_one(input), start.elapsed());

    let start = Instant::now();
    println!("Part two: {} in {:#?}", part_two(input), start.elapsed());
}

fn part_one(input: &str) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();

    // using this regex expression can capture the values in the brackets, the memory address and the value
    let reg = Regex::new(r"^mem+\[(\d+)\] = (\d+)$").unwrap();

    // the first number in the tuple will be all the bits that need to be set (stored as int)
    // second number is all the bits that need to be cleared (again stored as int)
    let mut mask = Mask {
        set: 0,
        clear: usize::MAX,
    };

    input.lines().for_each(|line| {
        if line.starts_with("ma") {
            mask = line
                .split(" = ")
                // ignore the first str in the split
                .nth(1)
                .unwrap()
                .bytes()
                // reverse so can operate on bits least significant to most significant
                .rev()
                .enumerate()
                // starting with default values, if a 0 is encountered at a bit then set the bit in the clear value
                // if a 1 is encountered then set this bit in the set value
                // if it isn't its an X so we can ignore it
                .fold(
                    Mask {
                        set: 0,
                        clear: usize::MAX,
                    },
                    |mask, (i, byte)| match byte {
                        b'0' => Mask {
                            set: mask.set,
                            clear: mask.clear & !(1 << i),
                        },
                        b'1' => Mask {
                            set: mask.set | 1 << i,
                            clear: mask.clear,
                        },
                        _ => Mask { ..mask },
                    },
                );
        } else {
            let instruction = reg.captures(line).unwrap();

            // the fist element in the capture is the whole string so need the next two
            memory.insert(
                instruction[1].parse::<usize>().unwrap(),
                // anding a 0 always gives 0, oring a 1 always gives 1, so sets the bits accordingly
                instruction[2].parse::<usize>().unwrap() & mask.clear | mask.set,
            );
        }
    });

    memory.values().sum()
}

fn part_two(input: &str) -> usize {
    let mut memory = HashMap::new();
    let mut mask: &[u8] = b"";

    let reg = Regex::new(r"^mem+\[(\d+)\] = (\d+)$").unwrap();

    #[rustfmt::skip]
    input
        .lines()
        .for_each(|line| if line.starts_with("ma") {
            mask = line
                    .split(" = ")
                    .nth(1)
                    .unwrap()
                    .as_bytes()
        } else {
            let values = reg.captures(line).unwrap();
            write(&mut memory, mask, values[1].parse().unwrap(), values[2].parse().unwrap(), 0);
        });

    memory.values().sum()
}

// recursive function over the bits in the mask
// base case: iterated over every bit, in this case accessing the address will give a None response
// otherwise match the mask bit
// if 0 call the recursive function but increment which bit is being looked at
// if 1 then set the bit at this index
// if x then call two recurisve functions, one with the bit the same and one with it toggled
fn write(memory: &mut HashMap<usize, usize>, mask: &[u8], address: usize, value: usize, i: usize) {
    match mask.get(i) {
        Some(b'0') => {
            write(memory, mask, address, value, i + 1);
        }
        Some(b'1') => {
            let bit = 1 << (35 - i);
            write(memory, mask, address | bit, value, i + 1);
        }
        Some(b'X') => {
            let bit = 1 << (35 - i);
            write(memory, mask, address, value, i + 1);
            write(memory, mask, address ^ bit, value, i + 1);
        }
        _ => {
            memory.insert(address, value);
        }
    }
}
