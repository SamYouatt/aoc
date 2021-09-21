use std::time::Instant;

fn main() {
    let mut instructions: Vec<(&[u8], bool, usize)> = include_bytes!("../../inputs/008.txt")
        .split(|b| b == &b'\n')
        .map(|line| {
            (
                &line[0..3],
                line[4] == b'-',
                std::str::from_utf8(&line[5..])
                    .unwrap()
                    .parse()
                    .expect("not number"),
            )
        })
        .collect();

    let (mut visited, mut pc, mut acc, mut start) = (vec![], 0, 0, Instant::now());

    while !visited.contains(&pc) {
        visited.push(pc);
        match instructions[pc] {
            (b"acc", is_neg, num) => {
                if is_neg {
                    acc -= num as isize;
                } else {
                    acc += num as isize;
                }
                pc += 1;
            }
            (b"jmp", is_neg, num) => {
                if is_neg {
                    pc -= num;
                } else {
                    pc += num;
                }
            }
            _ => {
                pc += 1;
            }
        }
    }

    println!("Part one: {}", acc);
    println!("Time taken: {:#?}", start.elapsed());

    start = Instant::now();
    let mut next_to_try = 0;

    while pc < 591 {
        match instructions[next_to_try] {
            (b"jmp", _, _) => {
                instructions[next_to_try].0 = b"nop";
                let result = run(&instructions);
                pc = result.0;
                acc = result.1;
                instructions[next_to_try].0 = b"jmp";
            }
            (b"nop", _, _) => {
                instructions[next_to_try].0 = b"jmp";
                let result = run(&instructions);
                pc = result.0;
                acc = result.1;
                instructions[next_to_try].0 = b"nop";
            }
            _ => {}
        }
        next_to_try += 1;
    }

    println!("Part two: {}", acc);
    println!("Time taken: {:#?}", start.elapsed());
}

fn run(program: &[(&[u8], bool, usize)]) -> (usize, isize) {
    let (mut visited, mut pc, mut acc) = (vec![], 0, 0);

    while !visited.contains(&pc) && pc < 591 {
        visited.push(pc);
        match program[pc] {
            (b"acc", is_neg, num) => {
                if is_neg {
                    acc -= num as isize;
                } else {
                    acc += num as isize;
                }
                pc += 1;
            }
            (b"jmp", is_neg, num) => {
                if is_neg {
                    pc -= num;
                } else {
                    pc += num;
                }
            }
            _ => {
                pc += 1;
            }
        }
    }

    (pc, acc)
}
