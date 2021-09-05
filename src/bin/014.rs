use std::collections::HashMap;

use regex::Regex;

struct Mask {
    set: usize,
    clear: usize,
}

fn main() {
    let input = include_str!("../../inputs/014.txt");
    let mut memory: HashMap<usize, usize> = HashMap::new();

    // using this regex expression can capture the values in the brackets, the memory address and the value
    let reg = Regex::new(r#"^mem\[(\d+)\] = (\d+)$"#).unwrap();

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
            let instruction = reg.captures(&line).unwrap();

            // the fist element in the capture is the whole string so need the next two
            memory.insert(
                instruction[1].parse::<usize>().unwrap(),
                // anding a 0 always gives 0, oring a 1 always gives 1, so sets the bits accordingly
                instruction[2].parse::<usize>().unwrap() & mask.clear | mask.set,
            );
        }
    });

    println!("Part one: {}", memory.values().sum::<usize>());
}
