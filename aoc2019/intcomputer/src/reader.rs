use std::io;

pub trait Reader {
    fn read_input() -> String;
}

pub struct StdInReader;

impl Reader for StdInReader {
    fn read_input() -> String {
        println!(">>");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        input.to_string()
    }
}
