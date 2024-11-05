use reader::Reader;
use writer::Writer;

pub mod reader;
pub mod writer;

type Tape = Vec<i64>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Parameter {
    /// Must be followed to a location
    Position(usize),
    /// Is the value to be used
    Immediate(i64),
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    /// 01 a + b -> c
    Add(Parameter, Parameter, usize),
    /// 02 a * b -> c
    Mult(Parameter, Parameter, usize),
    /// 03 loc
    Input(usize),
    /// 04 loc
    Output(Parameter),
}

pub struct Computer<R: Reader, W: Writer> {
    tape: Tape,
    head: usize,
    reader: R,
    writer: W,
}

impl<R: Reader, W: Writer> Computer<R, W> {
    pub fn load(tape: &Tape, reader: R, writer: W) -> Self {
        Self {
            tape: tape.to_owned(),
            head: 0,
            reader,
            writer,
        }
    }

    /// Dumps the current state of the tape
    pub fn dump_tape(&self) -> Tape {
        self.tape.to_owned()
    }

    /// Runs the program until it halts
    pub fn run(&mut self) {
        while let Some(instruction) = self.next_instruction() {
            match instruction {
                Instruction::Add(a, b, out) => {
                    let result = self.get_value(a) + self.get_value(b);
                    self.tape[out] = result;
                }
                Instruction::Mult(a, b, out) => {
                    let result = self.get_value(a) * self.get_value(b);
                    self.tape[out] = result;
                }
                Instruction::Input(dest) => {
                    let input = self.reader.read_input();
                    self.tape[dest] = input;
                }
                Instruction::Output(loc) => {
                    let value = self.get_value(loc);
                    self.writer.write_output(value);
                },
            }

            self.advance(&instruction);
        }
    }

    /// Parse the instruction at the current head, returns None for Halt
    fn next_instruction(&self) -> Option<Instruction> {
        match self.tape[self.head] {
            99 => None, // Halt
            1 => Some(Instruction::Add(
                Parameter::Position(self.tape[self.head + 1] as usize),
                Parameter::Position(self.tape[self.head + 2] as usize),
                self.tape[self.head + 3] as usize,
            )),
            2 => Some(Instruction::Mult(
                Parameter::Position(self.tape[self.head + 1] as usize),
                Parameter::Position(self.tape[self.head + 2] as usize),
                self.tape[self.head + 3] as usize,
            )),
            _ => panic!("unexpected instruction code"),
        }
    }

    fn get_value(&self, parameter: Parameter) -> i64 {
        match parameter {
            Parameter::Position(index) => self.tape[index],
            Parameter::Immediate(x) => x,
        }
    }

    /// Advance to the next instruction
    fn advance(&mut self, instruction: &Instruction) {
        let to_advance = match instruction {
            Instruction::Add(..) | Instruction::Mult(..) => 4,
            Instruction::Input(..) | Instruction::Output(..) => 2,
        };

        self.head += to_advance;
    }
}

/// Expects comma separated list of numbers
pub fn parse_tape(input: &str) -> Tape {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().expect("only numbers in input"))
        .collect()
}
