type Tape = Vec<usize>;

pub struct Computer {
    tape: Tape,
}

impl Computer {
    pub fn new(tape: &Tape) -> Self {
        Self {
            tape: tape.to_owned(),
        }
    }

    /// Dumps the current state of the tape
    pub fn dump_tape(&self) -> Tape {
        self.tape.to_owned()
    }

    /// Runs the program until it halts
    pub fn run(&mut self) {
        todo!()
    }
}

/// Expects comma separated list of numbers
pub fn parse_tape(input: &str) -> Tape {
    todo!()
}
