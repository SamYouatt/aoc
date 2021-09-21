use std::time::Instant;

fn main() {
    let (card_public, door_public) = include_str!("../../inputs/025.txt")
        .split_once("\n")
        .unwrap();
    let card_public = card_public.parse::<usize>().unwrap();
    let door_public = door_public.parse::<usize>().unwrap();

    let start = Instant::now();
    println!(
        "Part one: {} in {:#?}",
        part_one(card_public, door_public),
        start.elapsed()
    );
}

fn part_one(card_public: usize, door_public: usize) -> usize {
    let (mut card_public_new, mut door_public_new, mut card_encryption, mut door_encryption) =
        (1, 1, 1, 1);

    let mut answer = 0;

    while (card_public_new != card_public) && (door_public_new != door_public) {
        card_public_new = card_public_new * 7 % 20201227;
        door_public_new = door_public_new * 7 % 20201227;

        card_encryption = card_encryption * door_public % 20201227;
        door_encryption = door_encryption * card_public % 20201227;

        if card_public_new == card_public {
            answer = card_encryption;
        }

        if door_public_new == door_public {
            answer = door_encryption;
        }
    }

    answer
}
