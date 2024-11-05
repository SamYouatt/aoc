// Opcodes:
// 99 - Halt
// 1 - adds the two following reference's values and places in third's referenced location
// 2 - multiplies the two following reference's values and places in third's referenced location

type Tape = Vec<usize>;

pub struct Computer {
    tape: Tape,
    head: usize,
}

impl Computer {
    pub fn new(tape: &Tape) -> Self {
        Self {
            tape: tape.to_owned(),
            head: 0,
        }
    }

    /// Dumps the current state of the tape
    pub fn dump_tape(&self) -> Tape {
        self.tape.to_owned()
    }

    /// Runs the program until it halts
    pub fn run(&mut self) {
        while self.tape[self.head] != 99 {
            match self.tape[self.head] {
                1 => {
                    let first = self.deref_index(self.head + 1);
                    let second = self.deref_index(self.head + 2);

                    let result = first + second;

                    let result_loc = self.tape[self.head + 3];
                    self.tape[result_loc] = result;
                }
                2 => {
                    let first = self.deref_index(self.head + 1);
                    let second = self.deref_index(self.head + 2);

                    let result = first * second;

                    let result_loc = self.tape[self.head + 3];
                    self.tape[result_loc] = result;
                }
                99 => unreachable!(),
                _ => panic!("unsupported opcode"),
            }

            self.advance();
        }
    }

    /// Given a location in the tape, will inspect that value then follow it to the memory address
    fn deref_index(&self, reference_index: usize) -> usize {
        let reference = self.tape[reference_index];
        let deref = self.tape[reference];

        deref
    }

    /// Advance to the next opcode
    fn advance(&mut self) {
        self.head += 4;
    }
}

/// Expects comma separated list of numbers
pub fn parse_tape(input: &str) -> Tape {
    input
        .split(',')
        .map(|x| x.parse().expect("expected only integers in input"))
        .collect()
}
