pub trait Writer {
    fn write_output(output: &str);
}

pub struct StdOutWriter;

impl Writer for StdOutWriter {
    fn write_output(output: &str) {
        println!("{}", output);
    }
}
