use std::sync::mpsc::{Receiver, Sender};

pub mod days;

const MAX_MEMORY: usize = 10_000;
type Memory = [i64; MAX_MEMORY];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Parameter {
    /// Must be followed to a location
    Position(usize),
    /// Is the value to be used
    Immediate(i64),
    /// Like position but from the relative base
    Relative(i64),
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
    /// 05 cond loc
    JumpIfTrue(Parameter, Parameter),
    /// 06 cond loc
    JumpIfFalse(Parameter, Parameter),
    /// 07 a b loc
    LessThan(Parameter, Parameter, usize),
    /// 08 a b loc
    Equals(Parameter, Parameter, usize),
    /// 09 a
    AdjustRelativeBase(Parameter),
}

pub struct Computer {
    mem: Memory,
    instruction_ptr: usize,
    receiver: Receiver<i64>,
    sender: Sender<i64>,
    relative_base: usize,
}

impl Computer {
    pub fn load(tape: &[i64], receiver: Receiver<i64>, sender: Sender<i64>) -> Self {
        let mut mem = vec![0; 10_000];
        mem[0..tape.len()].clone_from_slice(tape);

        Self {
            mem: mem.try_into().expect("memory too big"),
            instruction_ptr: 0,
            receiver,
            sender,
            relative_base: 0,
        }
    }

    /// Dumps the current state of the tape
    pub fn dump_tape(&self) -> Memory {
        self.mem.to_owned()
    }

    pub fn receiver(&self) -> &Receiver<i64> {
        &self.receiver
    }

    /// Runs the program until it halts
    pub fn run(&mut self) {
        while let Some(instruction) = self.next_instruction() {
            let prev_instruction_ptr = self.instruction_ptr;

            match instruction {
                Instruction::Add(a, b, out) => {
                    let result = self.get_value(a) + self.get_value(b);
                    self.mem[out] = result;
                }
                Instruction::Mult(a, b, out) => {
                    let result = self.get_value(a) * self.get_value(b);
                    self.mem[out] = result;
                }
                Instruction::Input(dest) => {
                    let input = self.receiver.recv().expect("rec should never close");
                    self.mem[dest] = input;
                }
                Instruction::Output(loc) => {
                    let value = self.get_value(loc);
                    self.sender.send(value).expect("send should never close");
                }
                Instruction::JumpIfTrue(cond, loc) => {
                    if self.get_value(cond) != 0 {
                        self.instruction_ptr = self.get_value(loc) as usize;
                    }
                }
                Instruction::JumpIfFalse(cond, loc) => {
                    if self.get_value(cond) == 0 {
                        self.instruction_ptr = self.get_value(loc) as usize;
                    }
                }
                Instruction::LessThan(a, b, loc) => {
                    if self.get_value(a) < self.get_value(b) {
                        self.mem[loc] = 1;
                    } else {
                        self.mem[loc] = 0;
                    }
                }
                Instruction::Equals(a, b, loc) => {
                    if self.get_value(a) == self.get_value(b) {
                        self.mem[loc] = 1;
                    } else {
                        self.mem[loc] = 0;
                    }
                }
                Instruction::AdjustRelativeBase(a) => {
                    let test = (self.relative_base as i64 + self.get_value(a)) as usize;
                    self.relative_base = (self.relative_base as i64 + self.get_value(a)) as usize;
                }
            }

            if self.instruction_ptr == prev_instruction_ptr {
                self.advance(&instruction);
            }
        }
    }

    /// Parse the instruction at the current head, returns None for Halt
    fn next_instruction(&self) -> Option<Instruction> {
        let opcode = self.mem[self.instruction_ptr];
        let instruction_code = parse_opcode(opcode);
        match instruction_code {
            99 => None, // Halt
            1 => Some(Instruction::Add(
                parse_parameter(opcode, 1, self.mem[self.instruction_ptr + 1]),
                parse_parameter(opcode, 2, self.mem[self.instruction_ptr + 2]),
                self.mem[self.instruction_ptr + 3] as usize,
            )),
            2 => Some(Instruction::Mult(
                parse_parameter(opcode, 1, self.mem[self.instruction_ptr + 1]),
                parse_parameter(opcode, 2, self.mem[self.instruction_ptr + 2]),
                self.mem[self.instruction_ptr + 3] as usize,
            )),
            3 => Some(Instruction::Input(
                self.mem[self.instruction_ptr + 1] as usize,
            )),
            4 => Some(Instruction::Output(parse_parameter(
                opcode,
                1,
                self.mem[self.instruction_ptr + 1],
            ))),
            5 => Some(Instruction::JumpIfTrue(
                parse_parameter(opcode, 1, self.mem[self.instruction_ptr + 1]),
                parse_parameter(opcode, 2, self.mem[self.instruction_ptr + 2]),
            )),
            6 => Some(Instruction::JumpIfFalse(
                parse_parameter(opcode, 1, self.mem[self.instruction_ptr + 1]),
                parse_parameter(opcode, 2, self.mem[self.instruction_ptr + 2]),
            )),
            7 => Some(Instruction::LessThan(
                parse_parameter(opcode, 1, self.mem[self.instruction_ptr + 1]),
                parse_parameter(opcode, 2, self.mem[self.instruction_ptr + 2]),
                self.mem[self.instruction_ptr + 3] as usize,
            )),
            8 => Some(Instruction::Equals(
                parse_parameter(opcode, 1, self.mem[self.instruction_ptr + 1]),
                parse_parameter(opcode, 2, self.mem[self.instruction_ptr + 2]),
                self.mem[self.instruction_ptr + 3] as usize,
            )),
            9 => Some(Instruction::AdjustRelativeBase(parse_parameter(
                opcode,
                1,
                self.mem[self.instruction_ptr + 1],
            ))),
            _ => panic!("unexpected instruction code"),
        }
    }

    fn get_value(&self, parameter: Parameter) -> i64 {
        match parameter {
            Parameter::Position(index) => self.mem[index],
            Parameter::Immediate(x) => x,
            Parameter::Relative(offset) => self.mem[(self.relative_base as i64 + offset) as usize],
        }
    }

    /// Advance to the next instruction
    fn advance(&mut self, instruction: &Instruction) {
        let to_advance = match instruction {
            Instruction::Add(..)
            | Instruction::Mult(..)
            | Instruction::LessThan(..)
            | Instruction::Equals(..) => 4,
            Instruction::JumpIfTrue(..) | Instruction::JumpIfFalse(..) => 3,
            Instruction::Input(..)
            | Instruction::Output(..)
            | Instruction::AdjustRelativeBase(..) => 2,
        };

        self.instruction_ptr += to_advance;
    }
}

/// Expects comma separated list of numbers
pub fn parse_tape(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(|x| x.parse().expect("only numbers in input"))
        .collect()
}

fn parse_opcode(value: i64) -> usize {
    (value % 100) as usize
}

/// Get the parameter at position [1 based]
fn parse_parameter(opcode: i64, param_pos: usize, value: i64) -> Parameter {
    let flag = (opcode as f64 / 10f64.powf((param_pos + 1) as f64)).floor();
    if flag % 10.0 == 0.0 {
        Parameter::Position(value as usize)
    } else if flag % 2.0 == 0.0 {
        Parameter::Relative(value)
    } else {
        Parameter::Immediate(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_parameter_test() {
        let opcode = 1002;
        let value = 69;

        assert_eq!(
            parse_parameter(opcode, 1, value),
            Parameter::Position(value as usize)
        );
        assert_eq!(
            parse_parameter(opcode, 2, value),
            Parameter::Immediate(value)
        );
        assert_eq!(
            parse_parameter(opcode, 3, value),
            Parameter::Position(value as usize)
        );
    }

    #[test]
    fn parse_relative_test() {
        let opcode = 2002;
        let value = 69;

        assert_eq!(
            parse_parameter(opcode, 1, value),
            Parameter::Position(value as usize)
        );
        assert_eq!(
            parse_parameter(opcode, 2, value),
            Parameter::Relative(value)
        );
    }
}
