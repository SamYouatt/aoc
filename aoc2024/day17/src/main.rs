fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug, Clone)]
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

    while !puter.run() {}

    puter
        .out
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part_2(input: &str) -> String {
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

    let base_puter = Puter {
        ins_ptr: 0,
        reg_a: a,
        reg_b: b,
        reg_c: c,
        program: program.clone(),
        out: vec![],
    };

    // Observations:
    // ignore 0 its the same as 1 so consider 1 the base for searches
    //
    // the output length changes at increments based on init value
    // 1-7 => 1, 8-63 => 2, 64-511 => 3, 512-... => 4, etc...
    // which are powers of 8
    // 8^2 = 64 so 8^1 is start of 2 long
    // Program is 16 long so need 8 ^ 15 = 35trillion
    //
    // Program cycles its last digit cyclically as well
    // When 1 long stays for 1
    // When 2 long stays for 8
    // When 3 long stays for 64
    // so cycle on last number is 8^length
    // Second last number stays on the same cycle but one tier down
    // So stays on 0 in region 8>64 or 8^(length)
    //
    // From examining program:
    // what basically happens is:
    // while a != 0:
    //      take the last 3 bits of a
    //      do some stuff
    //      divide a by 8
    //      loop
    //
    // So what can do is start from a = 0 and iterate by 1 each time until the output
    // matches the last digit of the program
    // Then can multiply a by 8 and repeat the incrementation until the output matches the last 2
    // numbers
    // And then keep going until got the full 16 long program

    let mut start_a = 0;
    for matching_nums in 1..=16 {
        let a = find_suitable_a(start_a, matching_nums, &program, &base_puter);
        if matching_nums != 16 {
            start_a = 8 * a;
        } else {
            return a.to_string();
        }
    }

    unreachable!("never found answer");
}

fn find_suitable_a(
    start_a: usize,
    matched_nums: usize,
    program: &Vec<usize>,
    base_puter: &Puter,
) -> usize {
    let mut a = start_a;
    loop {
        let x = program.len() - matched_nums;
        let mut puter = base_puter.clone();
        puter.reg_a = a;

        while !puter.run() {}

        if puter.out == program[x..] {
            return a;
        }

        a += 1;
    }
}
