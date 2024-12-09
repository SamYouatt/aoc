fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
}

fn part_1(input: &str) -> usize {
    let mut disk = Vec::new();
    let mut file_id = 0_usize;

    for (i, char) in input.trim().chars().enumerate() {
        let digit = char.to_digit(10).unwrap();
        for _ in 0..digit {
            if i % 2 == 0 {
                disk.push(Some(file_id));
            } else {
                disk.push(None);
            }
        }

        if i % 2 == 0 {
            file_id += 1;
        }
    }

    let mut head_ptr = 0;
    let mut sorted_marker = disk.len() - 1;
    for tail_ptr in (0..disk.len()).rev() {
        sorted_marker -= 1;
        if disk[tail_ptr].is_none() {
            continue;
        }

        while head_ptr < disk.len() && disk[head_ptr].is_some() {
            head_ptr += 1;
        }

        if head_ptr == disk.len() {
            continue;
        }

        if head_ptr >= sorted_marker {
            break;
        }

        disk[head_ptr] = Some(disk[tail_ptr].unwrap());
        disk[tail_ptr] = None;
        head_ptr += 1;
    }

    disk.iter()
        .enumerate()
        .fold(0, |acc, (i, block)| acc + (block.unwrap_or(0) * i))
}
