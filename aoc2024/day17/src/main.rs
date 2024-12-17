fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

#[derive(Debug)]
struct Puter {
    ins_ptr: usize,
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    program: Vec<usize>,
    out: Vec<usize>,
}

impl Puter {
    fn run(&mut self) -> bool {
        let instruction = match self.program.get(self.ins_ptr) {
            Some(ins) => ins,
            None => return true,
        };
        let operand = self.program[self.ins_ptr + 1];

        match instruction {
            0 => {
                let numerator = self.reg_a;
                let denom = 2_isize.pow(self.combo(operand) as u32);
                self.reg_a = numerator / denom as usize;
                self.ins_ptr += 2;
            }
            1 => {
                self.reg_b = self.reg_b ^ operand;
                self.ins_ptr += 2;
            }
            2 => {
                let combo = self.combo(operand);
                self.reg_b = combo % 8;
                self.ins_ptr += 2;
            }
            3 => {
                if self.reg_a != 0 {
                    let jump = operand;
                    self.ins_ptr = jump;
                } else {
                    self.ins_ptr += 2;
                }
            }
            4 => {
                self.reg_b = self.reg_b ^ self.reg_c;
                self.ins_ptr += 2;
            }
            5 => {
                let combo = self.combo(operand);
                self.out.push(combo % 8);
                self.ins_ptr += 2;
            }
            6 => {
                let numerator = self.reg_a;
                let denom = 2_isize.pow(self.combo(operand) as u32);
                self.reg_b = numerator / denom as usize;
                self.ins_ptr += 2;
            }
            7 => {
                let numerator = self.reg_a;
                let denom = 2_isize.pow(self.combo(operand) as u32);
                self.reg_c = numerator / denom as usize;
                self.ins_ptr += 2;
            }
            x => unreachable!("invalid opcode {x}"),
        }

        false
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!("bad divide combo"),
        }
    }
}

fn part_1(input: &str) -> String {
    let (first, rest) = input.split_once('\n').unwrap();
    let (_, a) = first.split_once(": ").unwrap();
    let a = a.parse::<usize>().unwrap();

    let (second, rest) = rest.split_once('\n').unwrap();
    let (_, b) = second.split_once(": ").unwrap();
    let b = b.parse::<usize>().unwrap();

    let (third, rest) = rest.split_once("\n\n").unwrap();
    let (_, c) = third.split_once(": ").unwrap();
    let c = c.parse::<usize>().unwrap();

    let (_, program) = rest.split_once(": ").unwrap();
    let program = program
        .trim()
        .split(',')
        .map(|p| p.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut puter = Puter {
        ins_ptr: 0,
        reg_a: a,
        reg_b: b,
        reg_c: c,
        program,
        out: vec![],
    };

    loop {
        let done = puter.run();
        if done {
            break;
        }
    }

    puter
        .out
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}
