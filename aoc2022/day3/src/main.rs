fn main() {
    let input = include_str!("input.txt");

    let part1_answer = input
        .lines()
        .map(|rucksack| rucksack.chars().collect::<Vec<char>>())
        .map(|x| x.to_owned())
        .map(|items| {
            let halves = items.split_at(items.len() / 2);
            (halves.0.to_owned(), halves.1.to_owned())
        })
        .map(|(first, second)| find_intersect_item_as_value(&first, &second))
        .map(|item| get_item_value(item))
        .sum::<u32>();

    println!("Part 1: {part1_answer}");
}

fn find_intersect_item_as_value(first: &[char], second: &[char]) -> char {
    first
        .iter()
        .filter(|item_in_first| second.contains(item_in_first))
        .next()
        .expect("couldn't find interserct item")
        .to_owned()
}

fn get_item_value(item: char) -> u32 {
    let as_u32 = item as u32;
    if as_u32 > 97 {
        return as_u32 - 96;
    }

    return as_u32 - 38;
}
