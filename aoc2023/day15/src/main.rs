fn main() {
    let input = include_str!("input.txt");

    let answer1 = part_1(input);
    println!("Part 1: {answer1}");

    let answer2 = part_2(input);
    println!("Part 2: {answer2}");
}

fn part_1(input: &str) -> usize {
    input
        .trim()
        .split(",")
        .map(|sequence| {
            sequence
                .as_bytes()
                .iter()
                .fold(0, |total, ch| ((total + *ch as usize) * 17) % 256)
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    input
        .trim()
        .split(",")
        .for_each(|instruction| match instruction.contains("=") {
            true => {
                if let Some((label, lens)) = instruction.split_once('=') {
                    let lens = lens.parse::<usize>().unwrap();
                    let box_num = hash(label);

                    if let Some(existing_lens) = boxes[box_num].iter().position(|x| x.0 == label) {
                        let box_contents = boxes[box_num].get_mut(existing_lens).unwrap();
                        *box_contents = (label, lens);
                    } else {
                        boxes[box_num].push((label, lens));
                    }
                }
            }
            false => {
                let label = &instruction[0..instruction.len() - 1];
                let box_num = hash(label);

                if let Some(existing_lens) = boxes[box_num].iter().position(|x| x.0 == label) {
                    boxes[box_num].remove(existing_lens);
                }
            }
        });

    boxes.iter().enumerate().fold(0, |total, (box_num, boxx)| {
        total
            + boxx
                .iter()
                .enumerate()
                .fold(0, |sub_total, (lens_num, (_label, lens))| {
                    sub_total + (box_num + 1) * (lens_num + 1) * (lens)
                })
    })
}

fn hash(chars: &str) -> usize {
    chars
        .as_bytes()
        .iter()
        .fold(0, |total, &ch| ((total + ch as usize) * 17) % 256)
}
