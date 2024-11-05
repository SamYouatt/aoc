use std::io;

pub trait Reader {
    fn read_input(&mut self) -> i64;
}

pub struct StdInReader;

impl Reader for StdInReader {
    fn read_input(&mut self) -> i64 {
        println!(">>");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.parse().expect("input should be valid i64")
    }
}
