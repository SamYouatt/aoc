use intcomputer::{parse_tape, reader::Reader, writer::Writer, Computer};

struct TestReader {
    mock_input: i64,
}

impl TestReader {
    fn mock(mock_input: i64) -> Self {
        Self { mock_input }
    }
}

impl Reader for TestReader {
    fn read_input(&mut self) -> i64 {
        self.mock_input
    }
}

struct TestWriter {
    result: Option<i64>,
}

impl TestWriter {
    fn new() -> Self {
        Self { result: None }
    }
}

impl Writer for TestWriter {
    fn write_output(&mut self, output: i64) {
        self.result = Some(output);
    }
}

#[test]
fn first_example_equals_8() {
    let input = "3,9,8,9,10,9,4,9,99,-1,8";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(8);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(1));
}

#[test]
fn first_example_not_equals_8() {
    let input = "3,9,8,9,10,9,4,9,99,-1,8";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(25);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(0));
}

#[test]
fn second_example_less_than_8() {
    let input = "3,9,7,9,10,9,4,9,99,-1,8";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(4);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(1));
}

#[test]
fn second_example_not_less_than_8() {
    let input = "3,9,7,9,10,9,4,9,99,-1,8";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(25);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(0));
}

#[test]
fn third_example_equals_8() {
    let input = "3,3,1108,-1,8,3,4,3,99";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(8);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(1));
}

#[test]
fn third_example_not_equals_8() {
    let input = "3,3,1108,-1,8,3,4,3,99";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(25);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(0));
}

#[test]
fn fourth_example_less_than_8() {
    let input = "3,3,1107,-1,8,3,4,3,99";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(4);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(1));
}

#[test]
fn fourth_example_not_less_than_8() {
    let input = "3,3,1107,-1,8,3,4,3,99";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(25);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(0));
}

#[test]
fn beefy_example_input_below_8() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(3);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(999));
}

#[test]
fn beefy_example_input_equals_8() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(8);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(1000));
}

#[test]
fn beefy_example_input_greater_than_8() {
    let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    let tape = parse_tape(input);
    let mut test_reader = TestReader::mock(19);
    let mut test_writer = TestWriter::new();

    let mut computer = Computer::load(&tape, &mut test_reader, &mut test_writer);

    computer.run();

    assert_eq!(test_writer.result, Some(1001));
}
