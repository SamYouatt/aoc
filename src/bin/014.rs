fn main() {
    let input = include_str!("../../inputs/014.txt");

    // the first number in the tuple will be all the bits that need to be set (stored as int)
    // second number is all the bits that need to be cleared (again stored as int)
    let mut set_clear = (0, 0);

    input.lines().for_each(|line| {
        if line.starts_with("ma") {
            // update the mask
            set_clear = line
                .split(" = ")
                .nth(1)
                .unwrap()
                .bytes()
                .rev()
                .enumerate()
                .fold((usize::MAX, 0), |(set, clear), (i, byte)| {
                    println!("i: {}, byte: {}", i, byte);
                    match byte {
                        b'1' => (set & !(1 << i), clear),
                        b'0' => (set, clear | 1 << i),
                        _ => (set, clear),
                    }
                })
        } else {
            // get the memory address and value
            // apply the mask to the memory address
            // set the value at the memory address
        }
    });

    println!("set: {}, clear: {}", set_clear.0, set_clear.1);
}

fn apply_mask(mask: &[Option<bool>], memory_location: usize) -> usize {
    0
}
