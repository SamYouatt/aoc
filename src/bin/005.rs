use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("Part one: {}", part_one());
    let mut duration = start.elapsed();
    println!("Time taken: {:#?}", duration);
    println!("Part two: {}", part_two());
    duration = start.elapsed() - duration;
    println!("Time taken: {:#?}", duration);
}

// Important to realise that this problem can essentially boil down to setting binary digits based on the letter
// F and L are 0 bit
// R and B are 1 bit
// The last part of the id system is to do row * 8 + column
// This is the same as bit shifting right by 3 (creating space for 3 more bits) and setting the first 3 bits as the column
fn part_one() -> usize {
    include_str!("../../inputs/005.txt")
        .split("\n")
        .map(|seat| {
            seat.chars()
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

fn part_two() -> usize {
    let mut ids = include_str!("../../inputs/005.txt")
        .split("\n")
        .map(|seat| {
            seat.chars().fold(0 as usize, |id, char| {
                id << 1
                    | match char {
                        'B' | 'R' => 1,
                        'F' | 'L' => 0,
                        _ => panic!("❗"),
                    }
            })
        })
        .collect::<Vec<usize>>();

    // sorted so i can compare two ids next to each other
    ids.sort();

    // windows creates an iterator with array of elements with given size
    // looking at 2 values we can check the jump between them to find the missing value
    ids.windows(2)
        // find returns the first element which matches the criteria
        // in this case where the second element in the window is the same as the first + 2, i.e. the jump is 2
        // we can't just check if it greater than the other one by 2 because the ids might be missing at front or back
        // then we return the first element in that window + 1, the missing id
        .find(|window| window[1] == (window[0] + 2))
        .unwrap()[0]
        + 1
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
