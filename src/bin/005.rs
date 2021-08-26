fn main() {
    let answer1 = part_one();

    println!("Part one: {}", answer1);
}

// Important to realise that this problem can essentially boil down to setting binary digits based on the letter
// F and L are 0 bit
// R and B are 1 bit
// The last part of the id system is to do row * 8 + column
// This is the same as bit shifting right by 3 (creating space for 3 more bits) and setting the first 3 bits as the column
fn part_one() -> usize {
    // get puzzle input as &str
    include_str!("../../inputs/005.txt")
        // create an iterator with each element split on \n
        .split("\n")
        // apply transformation to each line (seat)
        .map(|seat| {
            seat
                // get iterator over the characters
                .chars()
                // fold gives an accumulator with an accumuatlor function
                .fold(0, |id, char| {
                    // move id left by 1 to set the next bit and or it to set the bit
                    // this is because new bit is created as 0 so we can just set it with an or operator
                    // if B or R then set it to 1, if F or L set it to 0
                    id << 1
                        | match char {
                            'B' | 'R' => 1,
                            'F' | 'L' => 0,
                            _ => panic!("❗"),
                        }
                })
        })
        // get the max value
        .max()
        .unwrap()
}

#[test]
fn test_part_one() {
    let answer = "BFFFBBFRRR".chars().fold(0, |id, char| {
        id << 1
            | match char {
                'B' | 'R' => 1,
                'F' | 'L' => 0,
                _ => panic!("❗"),
            }
    });
    assert_eq!(answer, 567)
}
