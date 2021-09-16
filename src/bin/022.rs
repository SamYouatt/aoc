use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::time::Instant;

enum Winner {
    Player1,
    Player2,
}

fn main() {
    let (deck1, deck2) = include_str!("../../inputs/022.txt")
        .split_once("\n\n")
        .unwrap();

    let deck1: VecDeque<usize> = deck1
        .lines()
        .skip(1)
        .map(|line| line.parse::<usize>().unwrap())
        .collect();
    let deck2: VecDeque<usize> = deck2
        .lines()
        .skip(1)
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let start = Instant::now();
    println!(
        "Part one: {} in {:#?}",
        part_one((&deck1, &deck2)),
        start.elapsed()
    );

    let start = Instant::now();
    println!(
        "Part two: {} in {:#?}",
        part_two((&deck1, &deck2)),
        start.elapsed()
    );
}

fn part_one(decks: (&VecDeque<usize>, &VecDeque<usize>)) -> usize {
    let mut deck1 = decks.0.clone();
    let mut deck2 = decks.1.clone();

    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        match card1.cmp(&card2) {
            Ordering::Less => {
                deck2.push_back(card2);
                deck2.push_back(card1);
            }
            Ordering::Greater => {
                deck1.push_back(card1);
                deck1.push_back(card2);
            }
            _ => {}
        }
    }

    deck1
        .iter()
        .chain(deck2.iter())
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i + 1))
        .sum::<usize>()
}

fn part_two(decks: (&VecDeque<usize>, &VecDeque<usize>)) -> usize {
    let (_, deck) = recurisve_combat(decks.0, decks.1);
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i + 1))
        .sum::<usize>()
}

fn recurisve_combat(deck1: &VecDeque<usize>, deck2: &VecDeque<usize>) -> (Winner, VecDeque<usize>) {
    let mut deck1 = deck1.clone();
    let mut deck2 = deck2.clone();

    let mut previous_states = HashSet::new();

    loop {
        // state has happened before, player 1 wins
        if !previous_states.insert((deck1.clone(), deck2.clone())) {
            return (Winner::Player1, deck1);
        }

        // if either is empty return the other winner
        if deck1.is_empty() {
            return (Winner::Player2, deck2);
        }

        if deck2.is_empty() {
            return (Winner::Player1, deck1);
        }

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if deck1.len() >= card1 && deck2.len() >= card2 {
            // recursive round
            let copied_deck1 = deck1.iter().take(card1).copied().collect();
            let copied_deck2 = deck2.iter().take(card2).copied().collect();

            match recurisve_combat(&copied_deck1, &copied_deck2) {
                (Winner::Player1, _) => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                (Winner::Player2, _) => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
            }
        } else {
            // highest card wins
            match card1.cmp(&card2) {
                Ordering::Less => {
                    deck2.push_back(card2);
                    deck2.push_back(card1);
                }
                Ordering::Greater => {
                    deck1.push_back(card1);
                    deck1.push_back(card2);
                }
                _ => {}
            }
        }
    }
}
