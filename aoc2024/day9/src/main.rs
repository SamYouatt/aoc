fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut disk = Vec::new();
    let mut file_id = 0_usize;

    for (i, char) in input.trim().chars().enumerate() {
        let digit = char.to_digit(10).unwrap();
        if i % 2 == 0 {
            disk.append(&mut [Some(file_id)].repeat(digit as usize));
            file_id += 1;
        } else {
            disk.append(&mut [None].repeat(digit as usize));
        }
    }

    for i in 0..disk.len() {
        if i >= disk.len() {
            break;
        }

        while disk.last().unwrap().is_none() {
            disk.remove(disk.len() - 1);
        }

        if disk[i].is_none() {
            disk.swap_remove(i);
        }
    }

    disk.iter()
        .enumerate()
        .fold(0, |acc, (i, block)| acc + (block.unwrap_or(0) * i))
}

struct File {
    id: usize,
    start: usize,
    len: usize,
}

struct Space {
    start: usize,
    len: usize,
}

fn part_2(input: &str) -> usize {
    let map = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut files = Vec::new();
    let mut file_id = 0;
    let mut spaces = Vec::new();
    let mut ptr = 0;

    for (i, len) in map.iter().enumerate() {
        if i % 2 == 0 {
            files.push(File {
                id: file_id,
                start: ptr,
                len: *len,
            });
            file_id += 1;
        } else {
            spaces.push(Space {
                start: ptr,
                len: *len,
            })
        }

        ptr += len;
    }

    for file in files.iter_mut().rev() {
        for space in spaces.iter_mut() {
            if space.len >= file.len && space.start < file.start {
                file.start = space.start;
                space.start += file.len;
                space.len -= file.len;
                break;
            }
        }
    }

    files.iter().fold(0, |acc, f| {
        acc + (f.start..(f.start + f.len)).fold(0, |i_acc, i| i_acc + (i * f.id))
    })
}
