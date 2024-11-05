pub trait Writer {
    fn write_output(&mut self, output: i64);
}

pub struct StdOutWriter;

impl Writer for StdOutWriter {
    fn write_output(&mut self, output: i64) {
        println!("{}", output);
    }
}
